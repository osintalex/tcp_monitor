use reqwest::blocking::Client;
use reqwest::{header, Error, StatusCode};
use serde_json::{json, Value};

use crate::settings::MonitorConfig;

/// Struct representing user fields for sendgrid payload
struct User {
    name: String,
    email: String,
}

/// Struct representing message fields for sendgrid payload
struct Message {
    subject: String,
    plain: String,
}

/// Send an alert email using the sendgrid api
///
/// # Arguments
///
/// * `ipv4_flag` - whether IPv4 check succeeded or not
/// * `ipv6_flag` - whether IPv6 check succeeded or not
/// * `configuration` - struct representing monitor configuration
///
pub fn send_alert_email(
    ipv4_flag: bool,
    ipv6_flag: bool,
    configuration: &MonitorConfig,
) -> Result<(), Error> {
    log::warn!("Sending an alert email...");
    let payload = make_email_payload(configuration, ipv4_flag, ipv6_flag);
    let api_key: &str = &configuration.sendgrid_apikey;

    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&payload)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

    let response = client.send()?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
            log::info!("Alert email sent!")
        }
        _ => log::error!(
            "Unable to send alert email. Status code was: {}. Body content was: {:?}",
            response.status(),
            response.text()
        ),
    }

    Ok(())
}

/// Generate a JSON payload for the sengrid api
///
/// # Arguments
///
/// * `ipv4_flag` - whether IPv4 check succeeded or not
/// * `ipv6_flag` - whether IPv6 check succeeded or not
/// * `configuration` - struct representing monitor configuration
///
fn make_email_payload(configuration: &MonitorConfig, ipv4_flag: bool, ipv6_flag: bool) -> Value {
    // first get all the data to put into the email
    let sender = User {
        name: String::from(&configuration.sender_name),
        email: String::from(&configuration.sender_email),
    };
    let recipient = User {
        name: String::from(&configuration.recipient_name),
        email: String::from(&configuration.recipient_email),
    };

    let ipv4_addr: &str = &configuration.ipv4;
    let ipv6_addr: &str = &configuration.ipv6;
    let ips_tuple = (ipv4_flag, ipv6_flag);
    let error_message: &str = "Tried to send an alert email when nothing was down!";
    let subject: &str = match ips_tuple {
        (false, false) => "IPv4 and IPv6 are down",
        (true, false) => "IPv4 is down",
        (false, true) => "IPv6 is down",
        _ => panic!("{}", error_message),
    };
    let plain: String = match ips_tuple {
        (false, false) => format!("{} and {} is down", ipv4_addr, ipv6_addr),
        (true, false) => format!("{} is down", ipv4_addr),
        (false, true) => format!("{} is down", ipv6_addr),
        _ => panic!("{}", error_message),
    };
    let message: Message = Message {
        subject: String::from(subject),
        plain,
    };

    // now convert all that data into a sendgrid api payload
    let payload = json!(
        {
            "personalizations": [{
                "from": {
                    "email": sender.email,
                    "name": sender.name
                },
                "to": [{
                    "email": recipient.email,
                    "name": recipient.name
                }]
            }],
            "from": {
                "email": sender.email,
                "name": sender.name
            },
            "subject": message.subject,
            "content": [
                {
                    "type": "text/plain",
                    "value": message.plain
                }
            ]
        }
    );
    payload
}

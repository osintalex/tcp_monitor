use log;
use simple_logger::SimpleLogger;

mod email;
mod settings;
mod tcp;

use crate::settings::MonitorConfig;

fn main() {
    SimpleLogger::new().init().unwrap();

    let configuration: MonitorConfig = settings::MonitorConfig::init();
    let ipv4_flag: bool = tcp::connect(&configuration.ipv4);
    let mut ipv6_flag: bool = true;
    if configuration.ipv6_enabled {
        ipv6_flag = tcp::connect(&configuration.ipv6);
    }
    if !ipv4_flag || !ipv6_flag {
        let alert_flag = settings::handle_failure(&configuration);
        if alert_flag {
            let result = email::send_alert_email(ipv4_flag, ipv6_flag, configuration);
            match result {
                Ok(_) => log::info!("Exiting after sending alert..."),
                Err(err) => log::error!("Error occurred sending the alert email: {:?}", err),
            }
        } else {
            log::info!("Exiting, there was an error but did not send an alert...")
        }
    } else {
        settings::handle_success();
        log::info!("Exiting without sending an alert, everything is up!")
    }
}

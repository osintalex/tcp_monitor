use config::Config;
use log;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::fs::{File, OpenOptions};
use std::path::Path;

const RESULTS_FILEPATH: &str = "src/settings/monitor_results.yml";
#[derive(Debug, Deserialize)]
pub struct MonitorConfig {
    pub ipv4: String,
    pub ipv6: String,
    pub ipv6_enabled: bool,
    pub sender_email: String,
    pub sender_name: String,
    pub recipient_email: String,
    pub recipient_name: String,
    pub seconds_between_checks: u64,
    pub sendgrid_apikey: String,
    how_many_failures_before_send_email: u8,
}

impl MonitorConfig {
    pub fn init() -> Self {
        let result = Config::builder()
            .add_source(config::File::with_name("settings"))
            .build()
            .unwrap()
            .try_deserialize::<MonitorConfig>();
        match result {
            Ok(x) => return x,
            Err(err) => panic!("Invalid settings file: {:?}", err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct MonitorResult {
    failed: bool,
    count: u8,
}

pub fn handle_failure(configuration: &MonitorConfig) -> bool {
    let mut results_yaml = read_results_yaml();
    results_yaml.failed = true;
    results_yaml.count += 1;
    write_results_yaml(&results_yaml);
    log::info!("{} unsuccessful result(s) so far...", results_yaml.count);
    if results_yaml.count >= configuration.how_many_failures_before_send_email {
        return true;
    }
    false
}

pub fn handle_success() {
    let mut results_yaml = read_results_yaml();
    results_yaml.failed = false;
    results_yaml.count = 0;
    write_results_yaml(&results_yaml);
}

fn read_results_yaml() -> MonitorResult {
    let file_result = File::open(&RESULTS_FILEPATH);
    let file = match file_result {
        Ok(file) => file,
        Err(why) => panic!(
            "Couldn't open {}: {}",
            Path::new(RESULTS_FILEPATH).display(),
            why
        ),
    };
    let read_result = from_reader(file);
    match read_result {
        Ok(read_result) => return read_result,
        Err(why) => panic!("Couldn't marshal to yaml {}", why),
    }
}

fn write_results_yaml(new_results_yaml: &MonitorResult) {
    let file_result = OpenOptions::new().write(true).open(RESULTS_FILEPATH);
    let path = Path::new(RESULTS_FILEPATH);
    let file = match file_result {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
    };
    let write_result = to_writer(file, new_results_yaml);
    match write_result {
        Ok(_) => log::info!("Wrote monitor test result to file..."),
        Err(why) => panic!("Couldn't write to {}: {}", path.display(), why),
    }
}

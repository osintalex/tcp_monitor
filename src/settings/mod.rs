use config::Config;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::fs::{File, OpenOptions};
use std::path::Path;

const RESULTS_FILEPATH: &str = "src/settings/monitor_results.yml";

/// Struct that the settings.toml file will get marshalled into
#[derive(Debug, Deserialize)]
pub struct MonitorConfig {
    pub email_enabled: bool,
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
    /// Read in the config and marshal it into the struct
    pub fn new() -> Self {
        let result = Config::builder()
            .add_source(config::File::with_name("settings"))
            .build()
            .unwrap()
            .try_deserialize::<MonitorConfig>();
        match result {
            Ok(x) => x,
            Err(err) => panic!("Invalid settings file: {:?}", err),
        }
    }
}

/// Struct that represents the yaml file
/// where results get tracked every program run
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorResult {
    failed: bool,
    count: u8,
}

impl MonitorResult {
    /// Write results to file when the check failed
    ///
    /// # Arguments
    ///
    /// * `configuration` - Struct representing all the configuration values for
    /// the application
    ///
    pub fn handle_failure(configuration: &MonitorConfig) -> bool {
        let mut results_yaml = MonitorResult::read_results_yaml();
        results_yaml.failed = true;
        results_yaml.count += 1;
        MonitorResult::write_results_yaml(&results_yaml);
        log::info!("{} unsuccessful result(s) so far...", results_yaml.count);
        if results_yaml.count >= configuration.how_many_failures_before_send_email {
            return true;
        }
        false
    }

    /// Write results to file when the check succeeds
    pub fn handle_success() {
        let mut results_yaml = Self::read_results_yaml();
        results_yaml.failed = false;
        results_yaml.count = 0;
        Self::write_results_yaml(&results_yaml);
    }

    /// Read the yaml file that contains the results
    fn read_results_yaml() -> MonitorResult {
        let file_result = File::open(RESULTS_FILEPATH);
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
            Ok(read_result) => read_result,
            Err(why) => panic!("Couldn't marshal to yaml {}", why),
        }
    }

    /// Write to the yaml file that contains the results
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
}

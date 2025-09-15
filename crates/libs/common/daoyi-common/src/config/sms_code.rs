use daoyi_common_support::utils::serde::deserialize_human_duration;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct SmsCodeConfig {
    #[serde(
        default = "default_expire_times",
        deserialize_with = "deserialize_human_duration"
    )]
    expire_times: Duration,
    #[serde(
        default = "default_send_frequency",
        deserialize_with = "deserialize_human_duration"
    )]
    send_frequency: Duration,
    #[serde(default = "default_send_maximum_quantity_per_day")]
    send_maximum_quantity_per_day: i32,
    #[serde(default = "default_begin_code")]
    begin_code: i32,
    #[serde(default = "default_end_code")]
    end_code: i32,
}

impl SmsCodeConfig {
    pub fn expire_times(&self) -> Duration {
        self.expire_times
    }
    pub fn send_frequency(&self) -> Duration {
        self.send_frequency
    }
    pub fn send_maximum_quantity_per_day(&self) -> i32 {
        self.send_maximum_quantity_per_day
    }
    pub fn begin_code(&self) -> i32 {
        self.begin_code
    }
    pub fn end_code(&self) -> i32 {
        self.end_code
    }
}

impl Default for SmsCodeConfig {
    fn default() -> Self {
        Self {
            expire_times: default_expire_times(),
            send_frequency: default_send_frequency(),
            send_maximum_quantity_per_day: default_send_maximum_quantity_per_day(),
            begin_code: default_begin_code(),
            end_code: default_end_code(),
        }
    }
}

fn default_expire_times() -> Duration {
    Duration::from_secs(10 * 60)
}

fn default_send_frequency() -> Duration {
    Duration::from_secs(60)
}

fn default_send_maximum_quantity_per_day() -> i32 {
    10
}
fn default_begin_code() -> i32 {
    100000
}
fn default_end_code() -> i32 {
    999999
}

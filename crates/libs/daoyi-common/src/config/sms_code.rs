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
}

fn default_expire_times() -> Duration {
    Duration::from_secs(10 * 60)
}

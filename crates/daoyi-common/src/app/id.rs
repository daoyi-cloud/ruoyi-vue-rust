use idgenerator::{IdGeneratorOptions, IdInstance};
use sea_orm::prelude::Date;

pub fn init() -> anyhow::Result<()> {
    let options = IdGeneratorOptions::new()
        .base_time(
            Date::from_ymd_opt(2025, 8, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
        )
        .worker_id(1)
        .worker_id_bit_len(5);
    Ok(IdInstance::init(options)?)
}

pub fn next_id() -> i64 {
    IdInstance::next_id()
}

pub fn next_id_str() -> String {
    next_id().to_string()
}

pub fn x() -> String {
    xid::new().to_string()
}

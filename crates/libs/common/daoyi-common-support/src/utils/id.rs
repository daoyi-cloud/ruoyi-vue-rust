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

// 生成固定长度的随机数字验证码
pub fn generate_sms_code(begin_code: i32, end_code: i32) -> String {
    // 生成指定范围内的随机数
    let random_number = rand::random_range(begin_code..=end_code);

    // 计算end_code的位数以确定验证码长度
    let code_length = end_code.to_string().len();

    // 格式化为固定长度的字符串，不足位数前面补0
    format!("{:0width$}", random_number, width = code_length)
}

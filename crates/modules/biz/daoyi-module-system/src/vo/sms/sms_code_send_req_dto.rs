use serde::{Deserialize, Serialize};
use validator::Validate;

/// 短信验证码的发送 Request DTO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SmsCodeSendReqDTO {
    /// 手机号
    pub mobile: String,
    /// 发送场景
    pub scene: i32,
    /// 发送 IP
    pub create_ip: String,
}

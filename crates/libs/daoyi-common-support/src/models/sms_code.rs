use crate::models::KeyValue;
use crate::support::tenant::TenantContextHolder;
use serde::{Deserialize, Serialize};

/// 短信发送消息结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsSendMessage {
    /// 短信日志编号
    pub log_id: i64,
    /// 手机号
    pub mobile: String,
    /// 短信渠道编号
    pub channel_id: i64,
    /// 短信API的模板编号
    pub api_template_id: String,
    /// 短信模板参数
    pub template_params: Vec<KeyValue<String, String>>,
    /// 租户
    pub tenant: TenantContextHolder,
}

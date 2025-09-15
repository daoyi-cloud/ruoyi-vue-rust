use crate::service::sms_channel::SmsChannelService;
use crate::service::sms_log::SmsLogService;
use crate::service::sms_template::SmsTemplateService;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::models::KeyValue;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::CommonStatusEnum;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{
    SMS_CHANNEL_NOT_EXISTS, SMS_SEND_MOBILE_NOT_EXISTS, SMS_SEND_MOBILE_TEMPLATE_PARAM_MISS,
    SMS_SEND_TEMPLATE_NOT_EXISTS,
};
use daoyi_common_support::utils::str_utils::format_template_content;
use daoyi_common_support::utils::web::validation::is_mobile_phone;
use daoyi_entities_system::entity::{system_sms_channel, system_sms_template};
use std::collections::HashMap;

pub struct SmsSendService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsSendService);
impl SmsSendService {
    pub async fn send_single_sms(
        &self,
        mobile: &str,
        user_id: Option<i64>,
        user_type: Option<i32>,
        template_code: &str,
        template_params: HashMap<&str, String>,
    ) -> ApiResult<i64> {
        // 校验短信模板是否合法
        let template = self.validate_sms_template(template_code).await?;
        // 校验短信渠道是否合法
        let channel = self.validate_sms_channel(template.channel_id).await?;
        // 校验手机号码是否存在
        self.validate_mobile(mobile).await?;
        // 构建有序的模板参数。为什么放在这个位置，是提前保证模板参数的正确性，而不是到了插入发送日志
        let new_template_params = self
            .build_template_params(&template, &template_params)
            .await?;
        // 创建发送日志。如果模板被禁用，则不发送短信，只记录日志
        let is_send = CommonStatusEnum::is_enable(template.status)
            && CommonStatusEnum::is_enable(channel.status);
        let content = format_template_content(&template.content, &template_params);
        let send_log_id = SmsLogService::new(self.tenant.clone())
            .create_sms_log(
                mobile,
                user_id,
                user_type,
                is_send,
                &template,
                content.as_ref(),
                &template_params,
            )
            .await?;
        // 发送 MQ 消息，异步执行发送短信
        Ok(send_log_id)
    }
    // 构建模板参数的函数
    async fn build_template_params(
        &self,
        template: &system_sms_template::Model,
        template_params: &HashMap<&str, String>,
    ) -> ApiResult<Vec<KeyValue<String, String>>> {
        let result = serde_json::from_str::<Vec<&str>>(template.params.as_ref())?
            .into_iter()
            .map(|key| match template_params.get(key) {
                Some(value) => Ok(KeyValue {
                    key: key.to_string(),
                    value: value.to_string(),
                }),
                None => Err(ApiError::BizCodeWithArgs(
                    SMS_SEND_MOBILE_TEMPLATE_PARAM_MISS,
                    vec![String::from(key)],
                )),
            })
            .collect::<ApiResult<Vec<KeyValue<String, String>>>>()?;
        Ok(result)
    }
    pub async fn validate_mobile(&self, mobile: &str) -> ApiResult<()> {
        // 验证手机号码
        if mobile.is_empty() {
            return Err(ApiError::BizCode(SMS_SEND_MOBILE_NOT_EXISTS));
        }
        is_mobile_phone(mobile)?;
        Ok(())
    }
    async fn validate_sms_channel(&self, channel_id: i64) -> ApiResult<system_sms_channel::Model> {
        let channel = SmsChannelService::new(self.tenant.clone())
            .get_sms_channel(channel_id)
            .await?;
        if channel.is_none() {
            return Err(ApiError::BizCode(SMS_CHANNEL_NOT_EXISTS));
        }
        Ok(channel.unwrap())
    }
    async fn validate_sms_template(
        &self,
        template_code: &str,
    ) -> ApiResult<system_sms_template::Model> {
        let template = SmsTemplateService::new(self.tenant.clone())
            .get_sms_template_by_code_from_cache(template_code)
            .await?;
        if template.is_none() {
            return Err(ApiError::BizCode(SMS_SEND_TEMPLATE_NOT_EXISTS));
        }
        Ok(template.unwrap())
    }
}

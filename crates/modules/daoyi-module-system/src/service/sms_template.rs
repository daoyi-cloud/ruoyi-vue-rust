use daoyi_common::app::{database, redis_util};
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::redis_key_constants::SMS_TEMPLATE;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_system::entity::prelude::SystemSmsTemplate;
use daoyi_entities_system::entity::system_sms_template;
use sea_orm::*;

pub struct SmsTemplateService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsTemplateService);
impl SmsTemplateService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemSmsTemplate> {
        SystemSmsTemplate::find().filter(system_sms_template::Column::Deleted.eq(0))
    }
}
impl SmsTemplateService {
    pub async fn get_sms_template_by_code_from_cache(
        &self,
        template_code: &str,
    ) -> ApiResult<Option<system_sms_template::Model>> {
        let key = format!("{SMS_TEMPLATE}:{template_code}");
        let cached = redis_util::cache_get_json::<system_sms_template::Model>(&key).await?;
        if cached.is_some() {
            return Ok(cached);
        }
        let template = self
            .base_query()
            .filter(system_sms_template::Column::Code.eq(template_code))
            .one(database::get()?)
            .await?;
        if template.is_some() {
            redis_util::cache_set_json_ex(
                &key,
                template.as_ref().unwrap(),
                60 * 60 * 24, // 1天
            )
            .await?;
        }
        Ok(template)
    }
}

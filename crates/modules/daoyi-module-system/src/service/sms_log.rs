use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_system::entity::prelude::SystemSmsLog;
use daoyi_entities_system::entity::{system_sms_log, system_sms_template};
use sea_orm::*;
use std::collections::HashMap;

pub struct SmsLogService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsLogService);
impl SmsLogService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemSmsLog> {
        SystemSmsLog::find().filter(system_sms_log::Column::Deleted.eq(0))
    }
}

impl SmsLogService {
    pub async fn create_sms_log(
        &self,
        mobile: &str,
        user_id: Option<i64>,
        user_type: Option<i32>,
        is_send: bool,
        template: &system_sms_template::Model,
        template_content: &str,
        template_params: &HashMap<&str, String>,
    ) -> ApiResult<i64> {
        todo!()
    }
}

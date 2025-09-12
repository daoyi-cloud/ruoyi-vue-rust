use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_system::entity::prelude::SystemSmsChannel;
use daoyi_entities_system::entity::system_sms_channel;
use sea_orm::*;

pub struct SmsChannelService {
    tenant: TenantContextHolder,
}

impl_tenant_instance!(SmsChannelService);
impl SmsChannelService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemSmsChannel> {
        SystemSmsChannel::find().filter(system_sms_channel::Column::Deleted.eq(0))
    }
}
impl SmsChannelService {
    pub async fn get_sms_channel(&self, id: i64) -> ApiResult<Option<system_sms_channel::Model>> {
        Ok(self
            .base_query()
            .filter(system_sms_channel::Column::Id.eq(id))
            .one(database::get()?)
            .await?)
    }
}

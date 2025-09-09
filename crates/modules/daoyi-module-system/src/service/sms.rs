use crate::vo::auth::SmsCodeSendReqDTO;
use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::SmsSceneEnum;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_entities_system::entity::prelude::SystemSmsCode;
use daoyi_entities_system::entity::{system_sms_code, system_users};
use sea_orm::*;

pub struct SmsCodeApi {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsCodeApi);
impl SmsCodeApi {
    pub async fn send_sms_code(&self, req_dto: SmsCodeSendReqDTO) -> ApiResult<()> {
        SmsCodeService::new(self.tenant.clone())
            .send_sms_code(req_dto)
            .await
    }
}
pub struct SmsCodeService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsCodeService);
impl SmsCodeService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemSmsCode> {
        SystemSmsCode::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Deleted.eq(0))
    }
}
impl SmsCodeService {
    pub async fn send_sms_code(&self, req_dto: SmsCodeSendReqDTO) -> ApiResult<()> {
        let scene_enum = SmsSceneEnum::from_scene(req_dto.scene)
            .ok_or_else(|| ApiError::Biz(format!("验证码场景({}) 查找不到配置", req_dto.scene)))?;
        // 创建验证码
        let code = self
            .create_sms_code(
                req_dto.mobile.as_ref(),
                req_dto.scene,
                req_dto.create_ip.as_ref(),
            )
            .await?;
        // 发送验证码
        Ok(())
    }
    async fn create_sms_code(&self, mobile: &str, scene: i32, ip: &str) -> ApiResult<String> {
        // 校验是否可以发送验证码，不用筛选场景
        let last_sms_code = self
            .base_query()
            .filter(system_sms_code::Column::Mobile.eq(mobile))
            .order_by_desc(system_sms_code::Column::Id)
            .one(database::get()?)
            .await?;
        if last_sms_code.is_some() {}
        Ok("123456".to_string())
    }
}

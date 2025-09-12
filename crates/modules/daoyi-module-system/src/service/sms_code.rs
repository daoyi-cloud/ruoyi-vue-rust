use crate::service::sms_send::SmsSendService;
use crate::vo::auth::SmsCodeSendReqDTO;
use daoyi_common::app::database;
use daoyi_common::{config, impl_tenant_instance};
use daoyi_common_support::support::orm::create_with_common_fields;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::SmsSceneEnum;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{
    SMS_CODE_EXCEED_SEND_MAXIMUM_QUANTITY_PER_DAY, SMS_CODE_SEND_TOO_FAST,
};
use daoyi_common_support::utils::id::generate_sms_code;
use daoyi_common_support::utils::is_today;
use daoyi_entities_system::entity::prelude::SystemSmsCode;
use daoyi_entities_system::entity::system_sms_code;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

pub struct SmsCodeService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsCodeService);
impl SmsCodeService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemSmsCode> {
        SystemSmsCode::find()
            .filter(system_sms_code::Column::TenantId.eq(self.tenant_id()))
            .filter(system_sms_code::Column::Deleted.eq(0))
    }
}
impl SmsCodeService {
    pub async fn send_sms_code(&self, req_dto: SmsCodeSendReqDTO) -> ApiResult<()> {
        let scene_enum = SmsSceneEnum::from_scene(req_dto.scene)
            .ok_or_else(|| ApiError::Biz(format!("验证码场景({}) 查找不到配置", req_dto.scene)))?;
        // 创建验证码
        let mobile = req_dto.mobile.as_ref();
        let code = self
            .create_sms_code(mobile, req_dto.scene, req_dto.create_ip.as_ref())
            .await?;
        // 发送验证码
        SmsSendService::new(self.tenant.clone())
            .send_single_sms(
                mobile,
                None,
                None,
                scene_enum.template_code(),
                HashMap::from([("code", code)]),
            )
            .await?;
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
        let sms_config = config::get().sms_code();
        let mut today_index = 1;
        if last_sms_code.is_some() {
            let last_sms_code = last_sms_code.unwrap();
            // 检查发送频率是否过于频繁
            let now = Local::now().naive_local();
            let create_time = last_sms_code.create_time;
            let duration = now.signed_duration_since(create_time).as_seconds_f64();

            // 获取配置中的发送频率限制（需要从配置中读取）
            let send_frequency = sms_config.send_frequency().as_secs() as f64; // 需要实现此方法
            if duration < send_frequency {
                return Err(ApiError::BizCode(SMS_CODE_SEND_TOO_FAST));
            }

            // 检查当天发送次数是否超过上限
            if is_today(&create_time)? && // 必须是今天，才能计算超过当天的上限
                last_sms_code.today_index >= sms_config.send_maximum_quantity_per_day()
            {
                // 超过当天发送的上限
                return Err(ApiError::BizCode(
                    SMS_CODE_EXCEED_SEND_MAXIMUM_QUANTITY_PER_DAY,
                ));
            }
            if is_today(&create_time)? {
                today_index = last_sms_code.today_index + 1;
            }
        }
        // 创建验证码记录
        let code = generate_sms_code(sms_config.begin_code(), sms_config.end_code());
        create_with_common_fields(
            system_sms_code::ActiveModel {
                mobile: Set(String::from(mobile)),
                code: Set(code.clone()),
                scene: Set(scene),
                today_index: Set(today_index),
                create_ip: Set(String::from(ip)),
                used: Set(false),
                ..Default::default()
            },
            None,
            &self.tenant,
        )
        .await?
        .insert(database::get()?)
        .await?;
        Ok(code)
    }
}

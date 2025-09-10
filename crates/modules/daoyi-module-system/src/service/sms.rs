use crate::vo::auth::SmsCodeSendReqDTO;
use daoyi_common::app::{database, redis_util};
use daoyi_common::{config, impl_tenant_instance};
use daoyi_common_support::support::orm::create_with_common_fields;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::SmsSceneEnum;
use daoyi_common_support::utils::enumeration::redis_key_constants::SMS_TEMPLATE;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{
    SMS_CHANNEL_NOT_EXISTS, SMS_CODE_EXCEED_SEND_MAXIMUM_QUANTITY_PER_DAY, SMS_CODE_SEND_TOO_FAST,
    SMS_SEND_MOBILE_NOT_EXISTS, SMS_SEND_TEMPLATE_NOT_EXISTS,
};
use daoyi_common_support::utils::id::generate_sms_code;
use daoyi_common_support::utils::is_today;
use daoyi_common_support::utils::web::validation::is_mobile_phone;
use daoyi_entities_system::entity::prelude::{
    SystemSmsChannel, SystemSmsCode, SystemSmsLog, SystemSmsTemplate,
};
use daoyi_entities_system::entity::{
    system_sms_channel, system_sms_code, system_sms_log, system_sms_template,
};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::*;
use std::collections::HashMap;

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
            .filter(system_sms_code::Column::TenantId.eq(self.tenant_id()))
            .filter(system_sms_code::Column::Deleted.eq(0))
    }
}
pub struct SmsSendService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SmsSendService);
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

impl SmsChannelService {
    pub async fn get_sms_channel(&self, id: i64) -> ApiResult<Option<system_sms_channel::Model>> {
        Ok(self
            .base_query()
            .filter(system_sms_channel::Column::Id.eq(id))
            .one(database::get()?)
            .await?)
    }
}
impl SmsSendService {
    pub async fn send_single_sms(
        &self,
        mobile: &str,
        user_id: Option<i64>,
        user_type: Option<i32>,
        template_code: &str,
        template_params: HashMap<&str, &str>,
    ) -> ApiResult<i64> {
        // 校验短信模板是否合法
        let template = self.validate_sms_template(template_code).await?;
        // 校验短信渠道是否合法
        let channel = self.validate_sms_channel(template.channel_id).await?;
        // 校验手机号码是否存在
        self.validate_mobile(mobile).await?;
        Ok(0)
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
        // tracing::info!(
        //     "发送验证码: scene_enum: {}, mobile: {}, code: {}",
        //     &scene_enum,
        //     &req_dto.mobile,
        //     &code
        // );
        SmsSendService::new(self.tenant.clone())
            .send_single_sms(
                mobile,
                None,
                None,
                scene_enum.template_code(),
                HashMap::from([("code", code.as_ref())]),
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

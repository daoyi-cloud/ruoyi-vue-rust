use crate::service::sms_code::SmsCodeService;
use crate::vo::sms::sms_code_send_req_dto::SmsCodeSendReqDTO;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::errors::error::ApiResult;

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

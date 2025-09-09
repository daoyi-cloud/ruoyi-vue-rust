use crate::app::utils::is_expired;
use crate::app::{TenantContextHolder, database, redis_util, utils::path_any_matches};
use crate::config;
use axum::body::Body;
use axum::http::{Request, Response};
use daoyi_common_support::utils::{
    enumeration,
    errors::{
        TENANT_DISABLE, TENANT_EXPIRE, TENANT_NOT_EXISTS,
        error::{ApiError, ApiResult},
    },
};
use daoyi_entities_system::entity::prelude::SystemTenant;
use sea_orm::prelude::*;
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static TENANT_LAYER: LazyLock<AsyncRequireAuthorizationLayer<TenantAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::<TenantAuth>::new(TenantAuth));

#[derive(Clone)]
pub struct TenantAuth;

impl AsyncAuthorizeRequest<Body> for TenantAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        Box::pin(async {
            let tenant = config::get().tenant();
            if !tenant.enabled() {
                request
                    .extensions_mut()
                    .insert(TenantContextHolder::default());
                return Ok(request);
            }
            let ignore_urls = tenant.ignore_urls();
            let tenant_id = request
                .headers()
                .get(tenant.header_name())
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(format!(
                                "{}请求头不是一个有效的数字",
                                tenant.header_name()
                            ))
                        })?
                        .parse::<i64>()
                        .map_err(|_| {
                            ApiError::Unauthenticated(format!(
                                "{}请求头不是一个有效的数字",
                                tenant.header_name()
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?;
            if tenant_id.is_none() && path_any_matches(&ignore_urls, request.uri().path())? {
                request
                    .extensions_mut()
                    .insert(TenantContextHolder::default());
                return Ok(request);
            }
            let tenant_id = tenant_id.ok_or_else(|| {
                ApiError::Unauthenticated(format!("{}请求头必须存在", tenant.header_name()))
            })?;
            valid_tenant(tenant_id).await?;
            request
                .extensions_mut()
                .insert(TenantContextHolder::set_tenant_id(tenant_id));
            Ok(request)
        })
    }
}

async fn valid_tenant(tenant_id: i64) -> ApiResult<()> {
    let cache_key = &format!("valid_tenant:{}", tenant_id);
    if let Some(t) = redis_util::cache_get::<bool>(cache_key).await? {
        if t {
            return Ok(());
        }
    }
    let db = database::get()?;
    let tenant = SystemTenant::find_by_id(tenant_id).one(db).await?;
    if tenant.is_none() {
        return Err(ApiError::BizCode(TENANT_NOT_EXISTS));
    }
    let tenant = tenant.unwrap();
    if enumeration::CommonStatusEnum::is_disable(tenant.status) {
        return Err(ApiError::BizCode(TENANT_DISABLE));
    }
    if is_expired(&tenant.expire_time)? {
        return Err(ApiError::BizCodeWithArgs(TENANT_EXPIRE, vec![tenant.name]));
    }
    redis_util::cache_set(cache_key, true).await?;
    Ok(())
}

pub fn get_tenant_layer() -> &'static AsyncRequireAuthorizationLayer<TenantAuth> {
    &TENANT_LAYER
}

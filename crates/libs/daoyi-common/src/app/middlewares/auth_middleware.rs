use crate::app::{
    TenantContextHolder,
    auth::{Auth, db_auth::get_default_db_auth, jsonwebtoken_auth::get_default_jwt},
    utils::path_any_matches,
};
use crate::config;
use axum::{
    body::Body,
    http::{Request, Response, header},
    response::IntoResponse,
};
use daoyi_common_support::utils::{enumeration, errors::error::ApiError};
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::<JWTAuth>::new(JWTAuth));

#[derive(Clone)]
pub struct JWTAuth;

impl AsyncAuthorizeRequest<Body> for JWTAuth {
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
            let auth_config = config::get().auth();
            let ignore_urls = auth_config.ignore_urls();
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization请求头不是一个有效的字符串",
                            ))
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization请求头必须以 Bearer 开头",
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?;
            if token.is_none() && path_any_matches(&ignore_urls, request.uri().path())? {
                return Ok(request);
            }
            let token = token.ok_or_else(|| {
                ApiError::Unauthenticated(String::from("Authorization请求头必须存在"))
            })?;
            let principal = match auth_config.method() {
                enumeration::AuthMethod::Jwt => get_default_jwt().decode(&token).await?,
                enumeration::AuthMethod::Db => get_default_db_auth().decode(&token).await?,
            };
            let tenant = request.extensions().get::<TenantContextHolder>();
            if let Some(tenant) = tenant {
                if !tenant.ignore() && tenant.tenant_id() != principal.tenant_id {
                    let error = ApiError::Unauthenticated(String::from("租户不匹配"));
                    return Err(error.into_response());
                }
            }
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
    &AUTH_LAYER
}

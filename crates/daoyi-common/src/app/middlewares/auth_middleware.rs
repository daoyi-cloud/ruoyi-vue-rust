use crate::app::utils::path_any_matches;
use crate::app::{
    TenantContextHolder, auth::jsonwebtoken_auth::get_default_jwt, errors::error::ApiError,
};
use crate::config;
use axum::body::Body;
use axum::http::{Request, Response, header};
use std::pin::Pin;
use std::sync::LazyLock;
use axum::response::IntoResponse;
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
            if token.is_none() && path_any_matches(&ignore_urls, &request.uri().path())? {
                return Ok(request);
            }
            let token = token.ok_or_else(|| {
                ApiError::Unauthenticated(String::from("Authorization请求头必须存在"))
            })?;
            let principal = get_default_jwt()
                .decode(&token)
                .map_err(|error| ApiError::Internal(error))?;
            let tenant = request
                .extensions()
                .get::<TenantContextHolder>();
            if let Some(tenant) = tenant {
                if !tenant.ignore() && tenant.tenant_id() != principal.tenant_id {

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

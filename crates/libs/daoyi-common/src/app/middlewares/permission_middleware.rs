use crate::app::auth::Principal;
use crate::app::{database, redis_util};
use axum::body::Body;
use axum::http::{Request, Response};
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::{RoleCode, redis_key_constants};
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_entities_system::entity::prelude::{
    SystemMenu, SystemRole, SystemRoleMenu, SystemUserRole,
};
use daoyi_entities_system::entity::{system_menu, system_role, system_role_menu, system_user_role};
use sea_orm::prelude::*;
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static PERMISSION_LAYER: LazyLock<AsyncRequireAuthorizationLayer<PermissionAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::<PermissionAuth>::new(PermissionAuth));

#[derive(Clone)]
pub struct PermissionAuth;

impl AsyncAuthorizeRequest<Body> for PermissionAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        Box::pin(async {
            let tenant = request.extensions().get::<TenantContextHolder>();
            if tenant.is_none() {
                return Ok(request);
            }
            let tenant = tenant.unwrap();
            let principal = request.extensions().get::<Principal>();
            if principal.is_none() {
                return Ok(request);
            }
            let principal = principal.unwrap();
            let api_path = request.uri().path();
            check_permission(tenant, principal, api_path).await?;
            Ok(request)
        })
    }
}

async fn check_permission(
    _tenant: &TenantContextHolder,
    principal: &Principal,
    api_path: &str,
) -> ApiResult<()> {
    let cache_key = format!(
        "{}:{}:{}",
        redis_key_constants::USER_HAS_PERMISSION,
        principal.user_id,
        api_path
    );
    let cached = redis_util::cache_get::<bool>(cache_key.as_ref()).await?;
    if cached.is_some() && cached.unwrap() {
        return Ok(());
    }
    let menus = SystemMenu::find()
        .filter(system_menu::Column::PermApis.contains(api_path))
        .all(database::get()?)
        .await?;
    if menus.is_empty() {
        redis_util::cache_set(cache_key.as_ref(), true).await?;
        return Ok(());
    }
    let user_roles = SystemUserRole::find()
        .filter(system_user_role::Column::UserId.eq(principal.user_id))
        .all(database::get()?)
        .await?;
    if user_roles.is_empty() {
        return Err(ApiError::Unauthenticated(format!(
            "接口[{api_path}]没有访问权限"
        )));
    }
    let role_ids: Vec<i64> = user_roles.iter().map(|r| r.role_id).collect();
    let menu_ids: Vec<i64> = menus.iter().map(|menu| menu.id).collect();
    let role_menus = SystemRoleMenu::find()
        .filter(system_role_menu::Column::MenuId.is_in(menu_ids))
        .all(database::get()?)
        .await?;
    if role_menus
        .iter()
        .any(|rm| (&role_ids).contains(&rm.role_id))
    {
        redis_util::cache_set(cache_key.as_ref(), true).await?;
        return Ok(());
    }
    let is_admin = SystemRole::find()
        .filter(system_role::Column::Id.is_in(role_ids))
        .all(database::get()?)
        .await?
        .iter()
        .any(|role| RoleCode::is_super_admin(role.code.as_ref()));
    if is_admin {
        redis_util::cache_set(cache_key.as_ref(), true).await?;
        return Ok(());
    }
    Err(ApiError::Unauthenticated(format!(
        "接口[{api_path}]没有访问权限"
    )))
}
pub fn get_permission_layer() -> &'static AsyncRequireAuthorizationLayer<PermissionAuth> {
    &PERMISSION_LAYER
}

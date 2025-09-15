use crate::service::menu::MenuService;
use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::RoleCode;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_system::entity::prelude::{SystemRoleMenu, SystemUserRole};
use daoyi_entities_system::entity::{system_role_menu, system_user_role};
use sea_orm::*;
use std::collections::HashSet;

pub struct PermissionService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(PermissionService);
impl PermissionService {
    pub async fn get_user_role_id_list_by_user_id(&self, user_id: i64) -> ApiResult<HashSet<i64>> {
        Ok(SystemUserRole::find()
            .filter(system_user_role::Column::UserId.eq(user_id))
            .all(database::get()?)
            .await?
            .into_iter()
            .map(|item| item.role_id)
            .collect::<HashSet<_>>())
    }

    pub async fn get_role_menu_list_by_role_id<C: AsRef<str>>(
        &self,
        role_ids: impl IntoIterator<Item = i64>,
        role_codes: impl IntoIterator<Item = C>,
    ) -> ApiResult<HashSet<i64>> {
        let mut peekable_role_ids = role_ids.into_iter().peekable();
        if peekable_role_ids.peek().is_none() {
            return Ok(HashSet::new());
        }
        // 如果是管理员的情况下，获取全部菜单编号
        if RoleCode::has_super_admin(role_codes) {
            return Ok(MenuService::new(self.tenant.clone())
                .get_all_menu_list()
                .await?
                .into_iter()
                .map(|item| item.id)
                .collect::<HashSet<_>>());
        }
        Ok(SystemRoleMenu::find()
            .filter(system_role_menu::Column::RoleId.is_in(peekable_role_ids))
            .all(database::get()?)
            .await?
            .into_iter()
            .map(|item| item.menu_id)
            .collect::<HashSet<_>>())
    }
}

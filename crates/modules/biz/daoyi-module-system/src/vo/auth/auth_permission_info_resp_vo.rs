use crate::vo::menu::menu_vo::MenuVo;
use crate::vo::user::user_vo::UserVo;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;

/// AuthPermissionInfoRespVO，管理后台 - 登录用户的权限信息 Response VO，额外包括用户信息和角色列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, ToSchema)]
pub struct AuthPermissionInfoRespVo {
    /// 菜单树
    pub menus: Vec<MenuVo>,
    /// 操作权限数组
    pub permissions: HashSet<String>,
    /// 角色标识数组
    pub roles: HashSet<String>,
    /// 用户信息
    pub user: UserVo,
}

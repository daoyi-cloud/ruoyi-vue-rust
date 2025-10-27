use daoyi_entities_system::entity::system_menu;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// MenuVO，管理后台 - 登录用户的菜单信息 Response VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MenuVo {
    /// 是否总是显示
    pub always_show: bool,
    /// 组件路径,仅菜单类型为菜单时，才需要传
    pub component: Option<String>,
    /// 组件名
    pub component_name: Option<String>,
    /// 菜单图标,仅菜单类型为菜单或者目录时，才需要传
    pub icon: Option<String>,
    /// 菜单名称
    pub id: i64,
    /// 是否缓存
    pub keep_alive: bool,
    /// 菜单名称
    pub name: String,
    /// 父菜单 ID
    pub parent_id: i64,
    /// 路由地址,仅菜单类型为菜单或者目录时，才需要传
    pub path: Option<String>,
    /// 是否可见
    pub visible: bool,
    /// 子路由
    pub children: Vec<MenuVo>,
}

impl From<system_menu::Model> for MenuVo {
    fn from(value: system_menu::Model) -> Self {
        Self {
            always_show: value.always_show,
            component: value.component,
            component_name: value.component_name,
            icon: value.icon,
            id: value.id,
            keep_alive: value.keep_alive,
            name: value.name,
            parent_id: value.parent_id,
            path: value.path,
            visible: value.visible,
            children: vec![],
        }
    }
}

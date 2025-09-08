use crate::impl_tenant_instance;
use crate::vo::auth::MenuVo;
use daoyi_common::app::database;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::MenuType;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_system::entity::prelude::SystemMenu;
use daoyi_entities_system::entity::system_menu;
use indextree::{Arena, NodeId};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::collections::HashMap;

pub struct MenuService {
    tenant: TenantContextHolder,
}

impl_tenant_instance!(MenuService);
impl MenuService {
    pub async fn get_all_menu_list(&self) -> ApiResult<Vec<system_menu::Model>> {
        Ok(SystemMenu::find().all(database::get()?).await?)
    }

    pub async fn get_menu_list(
        &self,
        menu_ids: impl IntoIterator<Item = i64>,
    ) -> ApiResult<Vec<system_menu::Model>> {
        Ok(SystemMenu::find()
            .filter(system_menu::Column::Id.is_in(menu_ids))
            .all(database::get()?)
            .await?)
    }
}

impl MenuService {
    pub async fn build_tree(list: Vec<system_menu::Model>) -> ApiResult<Vec<MenuVo>> {
        let mut list = list
            .into_iter()
            .filter(|m| m.r#type != MenuType::Button.get_type()) // 移除按钮
            .collect::<Vec<_>>();
        // 排序，保证菜单的有序性
        list.sort_by_key(|a| a.sort);

        // 创建 Arena 来存储节点
        let mut arena = Arena::new();
        let mut node_map: HashMap<i64, NodeId> = HashMap::new();
        let mut root_nodes: Vec<NodeId> = Vec::new();

        // 创建所有节点
        for menu in &list {
            let node_id = arena.new_node(menu.clone());
            node_map.insert(menu.id, node_id);
        }

        // 建立父子关系
        for menu in &list {
            let current_node = node_map[&menu.id];
            if menu.parent_id == 0 {
                // 顶级节点
                root_nodes.push(current_node);
            } else if let Some(parent_node_id) = node_map.get(&menu.parent_id) {
                // 添加为父节点的子节点
                parent_node_id.append(current_node, &mut arena);
            } else {
                // 找不到父节点，作为顶级节点处理
                root_nodes.push(current_node);
            }
        }

        // 转换为 MenuVo 结构
        let mut result = Vec::new();
        for root_node in root_nodes {
            if let Some(menu_vo) = Self::node_to_menu_vo(root_node, &arena, &node_map) {
                result.push(menu_vo);
            }
        }

        Ok(result)
    }

    fn node_to_menu_vo(
        node_id: NodeId,
        arena: &Arena<system_menu::Model>,
        node_map: &HashMap<i64, NodeId>,
    ) -> Option<MenuVo> {
        let node = arena[node_id].get();
        let mut menu_vo = MenuVo::from(node.clone());

        // 递归处理子节点
        for child_id in node_id.children(arena) {
            if let Some(child_menu_vo) = Self::node_to_menu_vo(child_id, arena, node_map) {
                menu_vo.children.push(child_menu_vo);
            }
        }

        Some(menu_vo)
    }
}

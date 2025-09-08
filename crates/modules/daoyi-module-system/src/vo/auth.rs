use daoyi_common_support::utils::serde::datetime_format;
use daoyi_entities_system::entity::{system_menu, system_oauth2_access_token, system_users};
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use validator::Validate;

/// AuthLoginReqVO，管理后台 - 账号密码登录 Request VO，如果登录并绑定社交用户，需要传递 social 开头的参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginReqVo {
    /// 验证码，验证码开启时，需要传递
    pub captcha_verification: Option<String>,
    /// 密码
    pub password: String,
    /// 授权码
    pub social_code: Option<String>,
    pub social_code_valid: Option<bool>,
    /// state
    pub social_state: Option<String>,
    /// 社交平台的类型，参见 SocialTypeEnum 枚举值
    pub social_type: Option<i32>,
    /// 账号
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthRefreshTokenReqVo {
    /// 刷新令牌
    pub refresh_token: String,
}

/// AuthLoginRespVO，管理后台 - 登录 Response VO
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginRespVo {
    /// 访问令牌
    pub access_token: String,
    /// 过期时间
    #[serde(with = "datetime_format")]
    pub expires_time: DateTime,
    /// 刷新令牌
    pub refresh_token: String,
    /// 用户编号
    pub user_id: i64,
}

impl From<system_oauth2_access_token::Model> for AuthLoginRespVo {
    fn from(value: system_oauth2_access_token::Model) -> Self {
        Self {
            access_token: value.access_token,
            expires_time: value.expires_time,
            refresh_token: value.refresh_token,
            user_id: value.user_id,
        }
    }
}

/// AuthPermissionInfoRespVO，管理后台 - 登录用户的权限信息 Response VO，额外包括用户信息和角色列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

/// MenuVO，管理后台 - 登录用户的菜单信息 Response VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

/// 用户信息
///
/// UserVO，用户信息 VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserVo {
    /// 用户头像
    pub avatar: Option<String>,
    /// 部门编号
    pub dept_id: Option<i64>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户编号
    pub id: i64,
    /// 用户昵称
    pub nickname: String,
    /// 用户账号
    pub username: String,
}

impl From<system_users::Model> for UserVo {
    fn from(model: system_users::Model) -> Self {
        Self {
            avatar: model.avatar,
            dept_id: model.dept_id,
            email: model.email,
            id: model.id,
            nickname: model.nickname,
            username: model.username,
        }
    }
}

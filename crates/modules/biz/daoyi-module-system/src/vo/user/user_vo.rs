use daoyi_entities_system::entity::system_users;
use serde::{Deserialize, Serialize};

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

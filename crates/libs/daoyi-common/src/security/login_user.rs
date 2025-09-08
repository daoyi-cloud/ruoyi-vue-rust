use chrono::{DateTime, Local};
use std::collections::HashMap;

/// 登录用户信息
#[derive(Debug, Clone, Default)]
pub struct LoginUser {
    /// 用户编号
    pub id: Option<i64>,

    /// 用户类型
    ///
    /// 关联 UserTypeEnum
    pub user_type: Option<i32>,

    /// 额外的用户信息
    pub info: HashMap<String, String>,

    /// 租户编号
    pub tenant_id: Option<i64>,

    /// 授权范围
    pub scopes: Vec<String>,

    /// 过期时间
    pub expires_time: Option<DateTime<Local>>,

    /// 上下文字段，不进行持久化
    ///
    /// 1. 用于基于 LoginUser 维度的临时缓存
    pub context: HashMap<String, String>,

    /// 访问的租户编号
    pub visit_tenant_id: Option<i64>,
}

impl LoginUser {
    pub const INFO_KEY_NICKNAME: &'static str = "nickname";
    pub const INFO_KEY_DEPT_ID: &'static str = "deptId";

    /// 设置上下文字段
    pub fn set_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    /// 获取上下文字段
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }
}

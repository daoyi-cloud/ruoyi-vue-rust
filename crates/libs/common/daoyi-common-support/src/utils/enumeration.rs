use sea_orm::{ActiveValue, IntoActiveValue, prelude::*};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub const EMPTY_VEC_STR: &'static str = "[]";

/// 可生成 T 数组的 trait
pub trait ArrayValuable<T> {
    /// 返回数组
    fn array(&self) -> Vec<T>;
}
/// 为类型实现ArrayValuable trait的宏
#[macro_export]
macro_rules! impl_array_valuable {
    ($type:ty, $return_type:ty, $array:expr) => {
        impl ArrayValuable<$return_type> for $type {
            fn array(&self) -> Vec<$return_type> {
                $array.to_vec()
            }
        }
    };
}

// 使用示例
// impl_array_valuable!(SocialTypeEnum, i32, [10, 20, 30, 31, 32, 34]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    Jwt,
    Db,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Gender {
    // #[sea_orm(string_value = "01")]
    // #[serde(rename = "male")]
    Male,
    // #[sea_orm(string_value = "02")]
    // #[serde(rename = "female")]
    Female,
}

impl IntoActiveValue<Gender> for Gender {
    fn into_active_value(self) -> ActiveValue<Gender> {
        ActiveValue::Set(self)
    }
}

/// 社交平台的类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialTypeEnum {
    /// Gitee
    ///
    /// 接入文档: https://gitee.com/api/v5/oauth_doc#/
    Gitee = 10,

    /// 钉钉
    ///
    /// 接入文档: https://developers.dingtalk.com/document/app/obtain-identity-credentials
    Dingtalk = 20,

    /// 企业微信
    ///
    /// 接入文档: https://xkcoding.com/2019/08/06/use-justauth-integration-wechat-enterprise.html
    WechatEnterprise = 30,

    /// 微信公众平台 - 移动端 H5
    ///
    /// 接入文档: https://www.cnblogs.com/juewuzhe/p/11905461.html
    WechatMp = 31,

    /// 微信开放平台 - 网站应用 PC 端扫码授权登录
    ///
    /// 接入文档: https://justauth.wiki/guide/oauth/wechat_open/#_2-申请开发者资质认证
    WechatOpen = 32,

    /// 微信小程序
    ///
    /// 接入文档: https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/login.html
    WechatMiniProgram = 34,
}

impl SocialTypeEnum {
    /// 获取类型
    pub fn get_type(&self) -> i32 {
        *self as i32
    }

    /// 获取类型的标识
    pub fn get_source(&self) -> &'static str {
        match self {
            SocialTypeEnum::Gitee => "GITEE",
            SocialTypeEnum::Dingtalk => "DINGTALK",
            SocialTypeEnum::WechatEnterprise => "WECHAT_ENTERPRISE",
            SocialTypeEnum::WechatMp => "WECHAT_MP",
            SocialTypeEnum::WechatOpen => "WECHAT_OPEN",
            SocialTypeEnum::WechatMiniProgram => "WECHAT_MINI_PROGRAM",
        }
    }

    /// 根据类型获取枚举值
    pub fn value_of_type(type_value: i32) -> Option<SocialTypeEnum> {
        match type_value {
            10 => Some(SocialTypeEnum::Gitee),
            20 => Some(SocialTypeEnum::Dingtalk),
            30 => Some(SocialTypeEnum::WechatEnterprise),
            31 => Some(SocialTypeEnum::WechatMp),
            32 => Some(SocialTypeEnum::WechatOpen),
            34 => Some(SocialTypeEnum::WechatMiniProgram),
            _ => None,
        }
    }

    /// 获取所有枚举值
    pub fn values() -> Vec<SocialTypeEnum> {
        vec![
            SocialTypeEnum::Gitee,
            SocialTypeEnum::Dingtalk,
            SocialTypeEnum::WechatEnterprise,
            SocialTypeEnum::WechatMp,
            SocialTypeEnum::WechatOpen,
            SocialTypeEnum::WechatMiniProgram,
        ]
    }
}

impl_array_valuable!(SocialTypeEnum, i32, [10, 20, 30, 31, 32, 34]);

/// 通用状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonStatusEnum {
    /// 开启
    Enable = 0,
    /// 关闭
    Disable = 1,
}

impl CommonStatusEnum {
    /// 获取状态值
    pub fn status(&self) -> i32 {
        *self as i32
    }

    /// 获取状态名
    pub fn name(&self) -> &'static str {
        match self {
            CommonStatusEnum::Enable => "开启",
            CommonStatusEnum::Disable => "关闭",
        }
    }

    /// 根据状态值获取对应的枚举
    pub fn from_status(status: i32) -> Option<CommonStatusEnum> {
        match status {
            0 => Some(CommonStatusEnum::Enable),
            1 => Some(CommonStatusEnum::Disable),
            _ => None,
        }
    }

    /// 判断是否为启用状态
    pub fn is_enable(status: i32) -> bool {
        status == CommonStatusEnum::Enable.status()
    }

    /// 判断是否为禁用状态
    pub fn is_disable(status: i32) -> bool {
        status == CommonStatusEnum::Disable.status()
    }
}
impl_array_valuable!(CommonStatusEnum, i32, [0, 1]);

/// 全局用户类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserTypeEnum {
    /// 会员 - 面向 c 端，普通用户
    Member = 1,
    /// 管理员 - 面向 b 端，管理后台
    Admin = 2,
}

impl FromStr for UserTypeEnum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "MEMBER" | "Member" | "member" => Ok(UserTypeEnum::Member),
            "2" | "ADMIN" | "Admin" | "admin" => Ok(UserTypeEnum::Admin),
            _ => Err(anyhow::anyhow!("Invalid UserTypeEnum value: {}", s)),
        }
    }
}

impl UserTypeEnum {
    /// 获取类型值
    pub fn value(&self) -> i32 {
        *self as i32
    }

    /// 获取类型名
    pub fn name(&self) -> &'static str {
        match self {
            UserTypeEnum::Member => "会员",
            UserTypeEnum::Admin => "管理员",
        }
    }

    /// 根据值查找对应的枚举 variant
    pub fn from_value(value: i32) -> Option<UserTypeEnum> {
        match value {
            1 => Some(UserTypeEnum::Member),
            2 => Some(UserTypeEnum::Admin),
            _ => None,
        }
    }

    /// 获取所有枚举值的数组
    pub fn values() -> Vec<UserTypeEnum> {
        vec![UserTypeEnum::Member, UserTypeEnum::Admin]
    }

    pub fn is_admin(value: i32) -> bool {
        value == UserTypeEnum::Admin.value()
    }

    pub fn is_member(value: i32) -> bool {
        value == UserTypeEnum::Member.value()
    }
}
impl Display for UserTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.value(), self.name())
    }
}
impl_array_valuable!(UserTypeEnum, i32, [1, 2]);

/// 登录日志的类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginLogTypeEnum {
    /// 使用账号登录
    LoginUsername = 100,
    /// 使用社交登录
    LoginSocial = 101,
    /// 使用手机登陆
    LoginMobile = 103,
    /// 使用短信登陆
    LoginSms = 104,
    /// 自己主动登出
    LogoutSelf = 200,
    /// 强制退出
    LogoutDelete = 202,
}

impl LoginLogTypeEnum {
    /// 获取日志类型
    pub fn type_value(&self) -> i32 {
        *self as i32
    }

    /// 根据值查找对应的枚举 variant
    pub fn from_type_value(value: i32) -> Option<LoginLogTypeEnum> {
        match value {
            100 => Some(LoginLogTypeEnum::LoginUsername),
            101 => Some(LoginLogTypeEnum::LoginSocial),
            103 => Some(LoginLogTypeEnum::LoginMobile),
            104 => Some(LoginLogTypeEnum::LoginSms),
            200 => Some(LoginLogTypeEnum::LogoutSelf),
            202 => Some(LoginLogTypeEnum::LogoutDelete),
            _ => None,
        }
    }

    /// 获取所有枚举值
    pub fn values() -> Vec<LoginLogTypeEnum> {
        vec![
            LoginLogTypeEnum::LoginUsername,
            LoginLogTypeEnum::LoginSocial,
            LoginLogTypeEnum::LoginMobile,
            LoginLogTypeEnum::LoginSms,
            LoginLogTypeEnum::LogoutSelf,
            LoginLogTypeEnum::LogoutDelete,
        ]
    }
}

/// OAuth2.0 客户端的通用常量
///
/// 对应 Java 中的 OAuth2ClientConstants 接口
pub mod oauth2_client_constants {
    /// 默认客户端 ID
    pub const CLIENT_ID_DEFAULT: &'static str = "default";
}

/// System Redis Key 常量
///
/// 对应 Java 中的 RedisKeyConstants 接口
pub mod redis_key_constants {
    /// 指定部门的所有子部门编号数组的缓存
    ///
    /// KEY 格式：dept_children_ids:{id}
    /// VALUE 数据类型：String 子部门编号集合
    pub const DEPT_CHILDREN_ID_LIST: &'static str = "dept_children_ids";

    /// 角色的缓存
    ///
    /// KEY 格式：role:{id}
    /// VALUE 数据类型：String 角色信息
    pub const ROLE: &'static str = "role";

    /// 用户拥有的角色编号的缓存
    ///
    /// KEY 格式：user_role_ids:{userId}
    /// VALUE 数据类型：String 角色编号集合
    pub const USER_ROLE_ID_LIST: &'static str = "user_role_ids";

    /// 拥有指定菜单的角色编号的缓存
    ///
    /// KEY 格式：user_role_ids:{menuId}
    /// VALUE 数据类型：String 角色编号集合
    pub const MENU_ROLE_ID_LIST: &'static str = "menu_role_ids";

    /// 拥有权限对应的菜单编号数组的缓存
    ///
    /// KEY 格式：permission_menu_ids:{permission}
    /// VALUE 数据类型：String 菜单编号数组
    pub const PERMISSION_MENU_ID_LIST: &'static str = "permission_menu_ids";

    /// 用户拥有的权限缓存
    pub const USER_HAS_PERMISSION: &'static str = "user_has_permission";

    /// OAuth2 客户端的缓存
    ///
    /// KEY 格式：oauth_client:{id}
    /// VALUE 数据类型：String 客户端信息
    pub const OAUTH_CLIENT: &'static str = "oauth_client";

    /// 访问令牌的缓存
    ///
    /// KEY 格式：oauth2_access_token:{token}
    /// VALUE 数据类型：String 访问令牌信息
    ///
    /// 由于动态过期时间，使用 RedisTemplate 操作
    pub const OAUTH2_ACCESS_TOKEN: &'static str = "oauth2_access_token";

    /// 站内信模版的缓存
    ///
    /// KEY 格式：notify_template:{code}
    /// VALUE 数据格式：String 模版信息
    pub const NOTIFY_TEMPLATE: &'static str = "notify_template";

    /// 邮件账号的缓存
    ///
    /// KEY 格式：mail_account:{id}
    /// VALUE 数据格式：String 账号信息
    pub const MAIL_ACCOUNT: &'static str = "mail_account";

    /// 邮件模版的缓存
    ///
    /// KEY 格式：mail_template:{code}
    /// VALUE 数据格式：String 模版信息
    pub const MAIL_TEMPLATE: &'static str = "mail_template";

    /// 短信模版的缓存
    ///
    /// KEY 格式：sms_template:{id}
    /// VALUE 数据格式：String 模版信息
    pub const SMS_TEMPLATE: &'static str = "sms_template";

    /// 小程序订阅模版的缓存
    ///
    /// KEY 格式：wxa_subscribe_template:{userType}
    /// VALUE 数据格式 String, 模版信息
    pub const WXA_SUBSCRIBE_TEMPLATE: &'static str = "wxa_subscribe_template";
}

/// 登录结果的枚举类
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginResultEnum {
    /// 成功
    Success = 0,
    /// 账号或密码不正确
    BadCredentials = 10,
    /// 用户被禁用
    UserDisabled = 20,
    /// 图片验证码不存在
    CaptchaNotFound = 30,
    /// 图片验证码不正确
    CaptchaCodeError = 31,
}

impl LoginResultEnum {
    /// 获取结果值
    pub fn result(&self) -> i32 {
        *self as i32
    }

    /// 根据值查找对应的枚举 variant
    pub fn from_result(value: i32) -> Option<LoginResultEnum> {
        match value {
            0 => Some(LoginResultEnum::Success),
            10 => Some(LoginResultEnum::BadCredentials),
            20 => Some(LoginResultEnum::UserDisabled),
            30 => Some(LoginResultEnum::CaptchaNotFound),
            31 => Some(LoginResultEnum::CaptchaCodeError),
            _ => None,
        }
    }

    /// 获取所有枚举值
    pub fn values() -> Vec<LoginResultEnum> {
        vec![
            LoginResultEnum::Success,
            LoginResultEnum::BadCredentials,
            LoginResultEnum::UserDisabled,
            LoginResultEnum::CaptchaNotFound,
            LoginResultEnum::CaptchaCodeError,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoleCode {
    SuperAdmin,
    TenantAdmin,
    CrmAdmin,
}

impl RoleCode {
    pub fn code(&self) -> &'static str {
        match self {
            RoleCode::SuperAdmin => "super_admin",
            RoleCode::TenantAdmin => "tenant_admin",
            RoleCode::CrmAdmin => "crm_admin",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RoleCode::SuperAdmin => "超级管理员",
            RoleCode::TenantAdmin => "租户管理员",
            RoleCode::CrmAdmin => "CRM 管理员",
        }
    }

    pub fn is_super_admin(code: &str) -> bool {
        code == "super_admin"
    }

    pub fn has_super_admin<V: AsRef<str>>(codes: impl IntoIterator<Item = V>) -> bool {
        codes
            .into_iter()
            .any(|code| RoleCode::is_super_admin(code.as_ref()))
    }
}

/// 菜单类型枚举类
///
/// 对应 Java 中的 cn.iocoder.yudao.module.system.enums.permission.MenuTypeEnum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuType {
    Dir = 1,    // 目录
    Menu = 2,   // 菜单
    Button = 3, // 按钮
}

impl MenuType {
    /// 获取菜单类型对应的整数值
    pub fn get_type(&self) -> i32 {
        *self as i32
    }
}

use std::fmt;

// 定义 SmsSceneEnum 枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SmsSceneEnum {
    MemberLogin,
    MemberUpdateMobile,
    MemberUpdatePassword,
    MemberResetPassword,
    AdminMemberLogin,
    AdminMemberRegister,
    AdminMemberResetPassword,
}

impl SmsSceneEnum {
    // 获取场景编号
    pub fn scene(&self) -> i32 {
        match self {
            SmsSceneEnum::MemberLogin => 1,
            SmsSceneEnum::MemberUpdateMobile => 2,
            SmsSceneEnum::MemberUpdatePassword => 3,
            SmsSceneEnum::MemberResetPassword => 4,
            SmsSceneEnum::AdminMemberLogin => 21,
            SmsSceneEnum::AdminMemberRegister => 22,
            SmsSceneEnum::AdminMemberResetPassword => 23,
        }
    }

    // 获取模板编码
    pub fn template_code(&self) -> &'static str {
        match self {
            SmsSceneEnum::MemberLogin => "user-sms-login",
            SmsSceneEnum::MemberUpdateMobile => "user-update-mobile",
            SmsSceneEnum::MemberUpdatePassword => "user-update-password",
            SmsSceneEnum::MemberResetPassword => "user-reset-password",
            SmsSceneEnum::AdminMemberLogin => "admin-sms-login",
            SmsSceneEnum::AdminMemberRegister => "admin-sms-register",
            SmsSceneEnum::AdminMemberResetPassword => "admin-reset-password",
        }
    }

    // 获取描述
    pub fn description(&self) -> &'static str {
        match self {
            SmsSceneEnum::MemberLogin => "会员用户 - 手机号登陆",
            SmsSceneEnum::MemberUpdateMobile => "会员用户 - 修改手机",
            SmsSceneEnum::MemberUpdatePassword => "会员用户 - 修改密码",
            SmsSceneEnum::MemberResetPassword => "会员用户 - 忘记密码",
            SmsSceneEnum::AdminMemberLogin => "后台用户 - 手机号登录",
            SmsSceneEnum::AdminMemberRegister => "后台用户 - 手机号注册",
            SmsSceneEnum::AdminMemberResetPassword => "后台用户 - 忘记密码",
        }
    }

    // 根据场景编号获取枚举值
    pub fn from_scene(scene: i32) -> Option<SmsSceneEnum> {
        match scene {
            1 => Some(SmsSceneEnum::MemberLogin),
            2 => Some(SmsSceneEnum::MemberUpdateMobile),
            3 => Some(SmsSceneEnum::MemberUpdatePassword),
            4 => Some(SmsSceneEnum::MemberResetPassword),
            21 => Some(SmsSceneEnum::AdminMemberLogin),
            22 => Some(SmsSceneEnum::AdminMemberRegister),
            23 => Some(SmsSceneEnum::AdminMemberResetPassword),
            _ => None,
        }
    }

    // 获取所有场景编号数组
    pub fn all_scenes() -> Vec<i32> {
        vec![1, 2, 3, 4, 21, 22, 23]
    }
}

// 实现 Display trait 以便打印描述
impl Display for SmsSceneEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sms_scene_enum() {
        let scene = SmsSceneEnum::from_scene(1).unwrap();
        assert_eq!(scene, SmsSceneEnum::MemberLogin);
        assert_eq!(scene.scene(), 1);
        assert_eq!(scene.template_code(), "user-sms-login");
        assert_eq!(scene.description(), "会员用户 - 手机号登陆");

        let admin_scene = SmsSceneEnum::from_scene(21).unwrap();
        assert_eq!(admin_scene, SmsSceneEnum::AdminMemberLogin);
        assert_eq!(admin_scene.scene(), 21);
        assert_eq!(admin_scene.template_code(), "admin-sms-login");
        assert_eq!(admin_scene.description(), "后台用户 - 手机号登录");
    }

    #[test]
    fn test_invalid_scene() {
        assert_eq!(SmsSceneEnum::from_scene(99), None);
    }

    #[test]
    fn test_all_scenes() {
        let scenes = SmsSceneEnum::all_scenes();
        assert_eq!(scenes.len(), 7);
        assert!(scenes.contains(&1));
        assert!(scenes.contains(&21));
    }
}

use std::fmt::{Display, Formatter};

pub mod error;

pub use error_code_constants::*;

/// 错误码对象
///
/// 全局错误码，占用 [0, 999]
/// 业务异常错误码，占用 [1_000_000_000, +∞)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCode {
    /// 错误码
    code: i32,
    /// 错误提示
    msg: &'static str,
}

impl ErrorCode {
    /// 创建新的错误码实例
    pub const fn new(code: i32, message: &'static str) -> Self {
        ErrorCode { code, msg: message }
    }

    /// 获取错误码
    pub fn code(&self) -> i32 {
        self.code
    }

    /// 获取错误提示
    pub fn msg(&self) -> &str {
        self.msg
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// System 错误码常量
///
/// system 系统，使用 1-002-000-000 段
pub mod error_code_constants {
    use super::ErrorCode;

    // ========== AUTH 模块 1-002-000-000 ==========
    pub const AUTH_LOGIN_BAD_CREDENTIALS: ErrorCode =
        ErrorCode::new(1002_000_000, "登录失败，账号密码不正确");
    pub const AUTH_LOGIN_USER_DISABLED: ErrorCode =
        ErrorCode::new(1_002_000_001, "登录失败，账号被禁用");
    pub const AUTH_LOGIN_CAPTCHA_CODE_ERROR: ErrorCode =
        ErrorCode::new(1_002_000_004, "验证码不正确，原因：{}");
    pub const AUTH_THIRD_LOGIN_NOT_BIND: ErrorCode =
        ErrorCode::new(1_002_000_005, "未绑定账号，需要进行绑定");
    pub const AUTH_MOBILE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_000_007, "手机号不存在");
    pub const AUTH_REGISTER_CAPTCHA_CODE_ERROR: ErrorCode =
        ErrorCode::new(1_002_000_008, "验证码不正确，原因：{}");

    // ========== 菜单模块 1-002-001-000 ==========
    pub const MENU_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_001_000, "已经存在该名字的菜单");
    pub const MENU_PARENT_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_001_001, "父菜单不存在");
    pub const MENU_PARENT_ERROR: ErrorCode = ErrorCode::new(1_002_001_002, "不能设置自己为父菜单");
    pub const MENU_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_001_003, "菜单不存在");
    pub const MENU_EXISTS_CHILDREN: ErrorCode =
        ErrorCode::new(1_002_001_004, "存在子菜单，无法删除");
    pub const MENU_PARENT_NOT_DIR_OR_MENU: ErrorCode =
        ErrorCode::new(1_002_001_005, "父菜单的类型必须是目录或者菜单");
    pub const MENU_COMPONENT_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_001_006, "已经存在该组件名的菜单");

    // ========== 角色模块 1-002-002-000 ==========
    pub const ROLE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_002_000, "角色不存在");
    pub const ROLE_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_002_001, "已经存在名为【{}】的角色");
    pub const ROLE_CODE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_002_002, "已经存在标识为【{}】的角色");
    pub const ROLE_CAN_NOT_UPDATE_SYSTEM_TYPE_ROLE: ErrorCode =
        ErrorCode::new(1_002_002_003, "不能操作类型为系统内置的角色");
    pub const ROLE_IS_DISABLE: ErrorCode =
        ErrorCode::new(1_002_002_004, "名字为【{}】的角色已被禁用");
    pub const ROLE_ADMIN_CODE_ERROR: ErrorCode =
        ErrorCode::new(1_002_002_005, "标识【{}】不能使用");

    // ========== 用户模块 1-002-003-000 ==========
    pub const USER_USERNAME_EXISTS: ErrorCode = ErrorCode::new(1_002_003_000, "用户账号已经存在");
    pub const USER_MOBILE_EXISTS: ErrorCode = ErrorCode::new(1_002_003_001, "手机号已经存在");
    pub const USER_EMAIL_EXISTS: ErrorCode = ErrorCode::new(1_002_003_002, "邮箱已经存在");
    pub const USER_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_003_003, "用户不存在");
    pub const USER_IMPORT_LIST_IS_EMPTY: ErrorCode =
        ErrorCode::new(1_002_003_004, "导入用户数据不能为空！");
    pub const USER_PASSWORD_FAILED: ErrorCode = ErrorCode::new(1_002_003_005, "用户密码校验失败");
    pub const USER_IS_DISABLE: ErrorCode =
        ErrorCode::new(1_002_003_006, "名字为【{}】的用户已被禁用");
    pub const USER_COUNT_MAX: ErrorCode = ErrorCode::new(
        1_002_003_008,
        "创建用户失败，原因：超过租户最大租户配额({})！",
    );
    pub const USER_IMPORT_INIT_PASSWORD: ErrorCode =
        ErrorCode::new(1_002_003_009, "初始密码不能为空");
    pub const USER_MOBILE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_003_010, "该手机号尚未注册");
    pub const USER_REGISTER_DISABLED: ErrorCode = ErrorCode::new(1_002_003_011, "注册功能已关闭");

    // ========== 部门模块 1-002-004-000 ==========
    pub const DEPT_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_004_000, "已经存在该名字的部门");
    pub const DEPT_PARENT_NOT_EXITS: ErrorCode = ErrorCode::new(1_002_004_001, "父级部门不存在");
    pub const DEPT_NOT_FOUND: ErrorCode = ErrorCode::new(1_002_004_002, "当前部门不存在");
    pub const DEPT_EXITS_CHILDREN: ErrorCode =
        ErrorCode::new(1_002_004_003, "存在子部门，无法删除");
    pub const DEPT_PARENT_ERROR: ErrorCode = ErrorCode::new(1_002_004_004, "不能设置自己为父部门");
    pub const DEPT_NOT_ENABLE: ErrorCode =
        ErrorCode::new(1_002_004_006, "部门({})不处于开启状态，不允许选择");
    pub const DEPT_PARENT_IS_CHILD: ErrorCode =
        ErrorCode::new(1_002_004_007, "不能设置自己的子部门为父部门");

    // ========== 岗位模块 1-002-005-000 ==========
    pub const POST_NOT_FOUND: ErrorCode = ErrorCode::new(1_002_005_000, "当前岗位不存在");
    pub const POST_NOT_ENABLE: ErrorCode =
        ErrorCode::new(1_002_005_001, "岗位({}) 不处于开启状态，不允许选择");
    pub const POST_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_005_002, "已经存在该名字的岗位");
    pub const POST_CODE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_005_003, "已经存在该标识的岗位");

    // ========== 字典类型 1-002-006-000 ==========
    pub const DICT_TYPE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_006_001, "当前字典类型不存在");
    pub const DICT_TYPE_NOT_ENABLE: ErrorCode =
        ErrorCode::new(1_002_006_002, "字典类型不处于开启状态，不允许选择");
    pub const DICT_TYPE_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_006_003, "已经存在该名字的字典类型");
    pub const DICT_TYPE_TYPE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_006_004, "已经存在该类型的字典类型");
    pub const DICT_TYPE_HAS_CHILDREN: ErrorCode =
        ErrorCode::new(1_002_006_005, "无法删除，该字典类型还有字典数据");

    // ========== 字典数据 1-002-007-000 ==========
    pub const DICT_DATA_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_007_001, "当前字典数据不存在");
    pub const DICT_DATA_NOT_ENABLE: ErrorCode =
        ErrorCode::new(1_002_007_002, "字典数据({})不处于开启状态，不允许选择");
    pub const DICT_DATA_VALUE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_007_003, "已经存在该值的字典数据");

    // ========== 通知公告 1-002-008-000 ==========
    pub const NOTICE_NOT_FOUND: ErrorCode = ErrorCode::new(1_002_008_001, "当前通知公告不存在");

    // ========== 短信渠道 1-002-011-000 ==========
    pub const SMS_CHANNEL_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_011_000, "短信渠道不存在");
    pub const SMS_CHANNEL_DISABLE: ErrorCode =
        ErrorCode::new(1_002_011_001, "短信渠道不处于开启状态，不允许选择");
    pub const SMS_CHANNEL_HAS_CHILDREN: ErrorCode =
        ErrorCode::new(1_002_011_002, "无法删除，该短信渠道还有短信模板");

    // ========== 短信模板 1-002-012-000 ==========
    pub const SMS_TEMPLATE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_012_000, "短信模板不存在");
    pub const SMS_TEMPLATE_CODE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_012_001, "已经存在编码为【{}】的短信模板");
    pub const SMS_TEMPLATE_API_ERROR: ErrorCode =
        ErrorCode::new(1_002_012_002, "短信 API 模板调用失败，原因是：{}");
    pub const SMS_TEMPLATE_API_AUDIT_CHECKING: ErrorCode =
        ErrorCode::new(1_002_012_003, "短信 API 模版无法使用，原因：审批中");
    pub const SMS_TEMPLATE_API_AUDIT_FAIL: ErrorCode =
        ErrorCode::new(1_002_012_004, "短信 API 模版无法使用，原因：审批不通过，{}");
    pub const SMS_TEMPLATE_API_NOT_FOUND: ErrorCode =
        ErrorCode::new(1_002_012_005, "短信 API 模版无法使用，原因：模版不存在");

    // ========== 短信发送 1-002-013-000 ==========
    pub const SMS_SEND_MOBILE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_013_000, "手机号不存在");
    pub const SMS_SEND_MOBILE_TEMPLATE_PARAM_MISS: ErrorCode =
        ErrorCode::new(1_002_013_001, "模板参数({})缺失");
    pub const SMS_SEND_TEMPLATE_NOT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_013_002, "短信模板不存在");

    // ========== 短信验证码 1-002-014-000 ==========
    pub const SMS_CODE_NOT_FOUND: ErrorCode = ErrorCode::new(1_002_014_000, "验证码不存在");
    pub const SMS_CODE_EXPIRED: ErrorCode = ErrorCode::new(1_002_014_001, "验证码已过期");
    pub const SMS_CODE_USED: ErrorCode = ErrorCode::new(1_002_014_002, "验证码已使用");
    pub const SMS_CODE_EXCEED_SEND_MAXIMUM_QUANTITY_PER_DAY: ErrorCode =
        ErrorCode::new(1_002_014_004, "超过每日短信发送数量");
    pub const SMS_CODE_SEND_TOO_FAST: ErrorCode = ErrorCode::new(1_002_014_005, "短信发送过于频繁");

    // ========== 租户信息 1-002-015-000 ==========
    pub const TENANT_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_015_000, "租户不存在");
    pub const TENANT_DISABLE: ErrorCode =
        ErrorCode::new(1_002_015_001, "名字为【{}】的租户已被禁用");
    pub const TENANT_EXPIRE: ErrorCode = ErrorCode::new(1_002_015_002, "名字为【{}】的租户已过期");
    pub const TENANT_CAN_NOT_UPDATE_SYSTEM: ErrorCode =
        ErrorCode::new(1_002_015_003, "系统租户不能进行修改、删除等操作！");
    pub const TENANT_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_015_004, "名字为【{}】的租户已存在");
    pub const TENANT_WEBSITE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_015_005, "域名为【{}】的租户已存在");

    // ========== 租户套餐 1-002-016-000 ==========
    pub const TENANT_PACKAGE_NOT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_016_000, "租户套餐不存在");
    pub const TENANT_PACKAGE_USED: ErrorCode = ErrorCode::new(
        1_002_016_001,
        "租户正在使用该套餐，请给租户重新设置套餐后再尝试删除",
    );
    pub const TENANT_PACKAGE_DISABLE: ErrorCode =
        ErrorCode::new(1_002_016_002, "名字为【{}】的租户套餐已被禁用");
    pub const TENANT_PACKAGE_NAME_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_016_003, "已经存在该名字的租户套餐");

    // ========== 社交用户 1-002-018-000 ==========
    pub const SOCIAL_USER_AUTH_FAILURE: ErrorCode =
        ErrorCode::new(1_002_018_000, "社交授权失败，原因是：{}");
    pub const SOCIAL_USER_NOT_FOUND: ErrorCode =
        ErrorCode::new(1_002_018_001, "社交授权失败，找不到对应的用户");

    pub const SOCIAL_CLIENT_WEIXIN_MINI_APP_PHONE_CODE_ERROR: ErrorCode =
        ErrorCode::new(1_002_018_200, "获得手机号失败");
    pub const SOCIAL_CLIENT_WEIXIN_MINI_APP_QRCODE_ERROR: ErrorCode =
        ErrorCode::new(1_002_018_201, "获得小程序码失败");
    pub const SOCIAL_CLIENT_WEIXIN_MINI_APP_SUBSCRIBE_TEMPLATE_ERROR: ErrorCode =
        ErrorCode::new(1_002_018_202, "获得小程序订阅消息模版失败");
    pub const SOCIAL_CLIENT_WEIXIN_MINI_APP_SUBSCRIBE_MESSAGE_ERROR: ErrorCode =
        ErrorCode::new(1_002_018_203, "发送小程序订阅消息失败");
    pub const SOCIAL_CLIENT_WEIXIN_MINI_APP_ORDER_UPLOAD_SHIPPING_INFO_ERROR: ErrorCode =
        ErrorCode::new(1_002_018_204, "上传微信小程序发货信息失败");
    pub const SOCIAL_CLIENT_WEIXIN_MINI_APP_ORDER_NOTIFY_CONFIRM_RECEIVE_ERROR: ErrorCode =
        ErrorCode::new(1_002_018_205, "上传微信小程序订单收货信息失败");
    pub const SOCIAL_CLIENT_NOT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_018_210, "社交客户端不存在");
    pub const SOCIAL_CLIENT_UNIQUE: ErrorCode =
        ErrorCode::new(1_002_018_211, "社交客户端已存在配置");

    // ========== OAuth2 客户端 1-002-020-000 =========
    pub const OAUTH2_CLIENT_NOT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_020_000, "OAuth2 客户端不存在");
    pub const OAUTH2_CLIENT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_020_001, "OAuth2 客户端编号已存在");
    pub const OAUTH2_CLIENT_DISABLE: ErrorCode =
        ErrorCode::new(1_002_020_002, "OAuth2 客户端已禁用");
    pub const OAUTH2_CLIENT_AUTHORIZED_GRANT_TYPE_NOT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_020_003, "不支持该授权类型");
    pub const OAUTH2_CLIENT_SCOPE_OVER: ErrorCode = ErrorCode::new(1_002_020_004, "授权范围过大");
    pub const OAUTH2_CLIENT_REDIRECT_URI_NOT_MATCH: ErrorCode =
        ErrorCode::new(1_002_020_005, "无效 redirect_uri: {}");
    pub const OAUTH2_CLIENT_CLIENT_SECRET_ERROR: ErrorCode =
        ErrorCode::new(1_002_020_006, "无效 client_secret: {}");

    // ========== OAuth2 授权 1-002-021-000 =========
    pub const OAUTH2_GRANT_CLIENT_ID_MISMATCH: ErrorCode =
        ErrorCode::new(1_002_021_000, "client_id 不匹配");
    pub const OAUTH2_GRANT_REDIRECT_URI_MISMATCH: ErrorCode =
        ErrorCode::new(1_002_021_001, "redirect_uri 不匹配");
    pub const OAUTH2_GRANT_STATE_MISMATCH: ErrorCode =
        ErrorCode::new(1_002_021_002, "state 不匹配");

    // ========== OAuth2 授权 1-002-022-000 =========
    pub const OAUTH2_CODE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_022_000, "code 不存在");
    pub const OAUTH2_CODE_EXPIRE: ErrorCode = ErrorCode::new(1_002_022_001, "code 已过期");

    // ========== 邮箱账号 1-002-023-000 ==========
    pub const MAIL_ACCOUNT_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_023_000, "邮箱账号不存在");
    pub const MAIL_ACCOUNT_RELATE_TEMPLATE_EXISTS: ErrorCode =
        ErrorCode::new(1_002_023_001, "无法删除，该邮箱账号还有邮件模板");

    // ========== 邮件模版 1-002-024-000 ==========
    pub const MAIL_TEMPLATE_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_024_000, "邮件模版不存在");
    pub const MAIL_TEMPLATE_CODE_EXISTS: ErrorCode =
        ErrorCode::new(1_002_024_001, "邮件模版 code({}) 已存在");

    // ========== 邮件发送 1-002-025-000 ==========
    pub const MAIL_SEND_TEMPLATE_PARAM_MISS: ErrorCode =
        ErrorCode::new(1_002_025_000, "模板参数({})缺失");
    pub const MAIL_SEND_MAIL_NOT_EXISTS: ErrorCode = ErrorCode::new(1_002_025_001, "邮箱不存在");

    // ========== 站内信模版 1-002-026-000 ==========
    pub const NOTIFY_TEMPLATE_NOT_EXISTS: ErrorCode =
        ErrorCode::new(1_002_026_000, "站内信模版不存在");
    pub const NOTIFY_TEMPLATE_CODE_DUPLICATE: ErrorCode =
        ErrorCode::new(1_002_026_001, "已经存在编码为【{}】的站内信模板");

    // ========== 站内信发送 1-002-028-000 ==========
    pub const NOTIFY_SEND_TEMPLATE_PARAM_MISS: ErrorCode =
        ErrorCode::new(1_002_028_000, "模板参数({})缺失");
}

use sea_orm::{ActiveValue, IntoActiveValue, prelude::*};
use serde::{Deserialize, Serialize};

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

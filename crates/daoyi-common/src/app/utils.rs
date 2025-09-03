use crate::app::{
    error::{ApiError, ApiResult},
    id,
};
use std::sync::LazyLock;
use wax::{Glob, Pattern};

pub static RANDOM_PASSWORD: LazyLock<String> =
    LazyLock::new(|| encode_password(&id::next_id()).unwrap_or_default());
#[allow(dead_code)]
pub fn encode_password(password: &str) -> ApiResult<String> {
    Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
}

#[allow(dead_code)]
pub fn verify_password(password: &str, hashed_password: &str) -> ApiResult<bool> {
    Ok(bcrypt::verify(password, hashed_password)?)
}

pub fn path_matches(pattern: &str, target: &str) -> ApiResult<bool> {
    // 将通配符模式编译为 Glob 表达式
    let glob =
        Glob::new(pattern).map_err(|_| ApiError::Biz(String::from("Invalid glob pattern")))?;
    // 判断目标路径是否匹配该模式
    Ok(glob.is_match(target))
}

pub fn path_any_matches(patterns: &[String], target: &str) -> ApiResult<bool> {
    for pattern in patterns {
        if path_matches(pattern, target)? {
            return Ok(true);
        }
    }
    Ok(false)
}

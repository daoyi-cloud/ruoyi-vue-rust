use crate::app::error::ApiResult;
use crate::app::id;
use std::sync::LazyLock;

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

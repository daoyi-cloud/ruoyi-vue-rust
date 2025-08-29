use regex::Regex;
use std::borrow::Cow;
use std::cell::LazyCell;
use std::collections::HashMap;
use validator::ValidationError;

const MOBILE_PHONE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^\+?\d{1,4}?[-.\s]?\(?\d{1,4}\)?([-.\s]?\d{1,9}){1,4}$")
        .expect("Failed to compile mobile phone regex")
});
// const MOBILE_PHONE_REGEX: LazyCell<Regex> =
//     LazyCell::new(|| Regex::new(r"^1[3-9]\d{9}$").expect("Failed to compile mobile phone regex"));
pub fn is_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if !MOBILE_PHONE_REGEX.is_match(value) {
        return Err(build_validation_error("手机号码格式不正确"));
    }
    Ok(())
}

fn build_validation_error(message: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(message)),
        params: HashMap::new(),
    }
}

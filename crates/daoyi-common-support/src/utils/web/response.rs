use crate::utils::errors::ErrorCode;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }

    pub fn ok<M: AsRef<str>>(message: M, data: Option<T>) -> Self {
        Self::new(0, String::from(message.as_ref()), data)
    }

    pub fn err<M: AsRef<str>>(message: M) -> Self {
        Self::new(1, String::from(message.as_ref()), None)
    }

    pub fn biz_err(error_code: ErrorCode) -> Self {
        Self::new(error_code.code(), String::from(error_code.msg()), None)
    }

    pub fn biz_err_with_args<M: AsRef<str> + Display>(error_code: ErrorCode, args: Vec<M>) -> Self {
        Self::new(
            error_code.code(),
            String::from(error_code.format_message(args)),
            None,
        )
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

use super::ErrorCode;
use crate::app::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;
use sea_orm::DbErr;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ApiJsonResult<T> = ApiResult<ApiResponse<T>>;

#[allow(dead_code)]
pub fn api_json_ok<T>(data: T) -> ApiJsonResult<T> {
    api_json_msg_ok("ok", data)
}

#[allow(dead_code)]
pub fn api_json_msg_ok<M: AsRef<str>, T>(message: M, data: T) -> ApiJsonResult<T> {
    Ok(ApiResponse::ok(message, Some(data)))
}

#[allow(dead_code)]
pub fn api_empty_ok() -> ApiJsonResult<()> {
    Ok(ApiResponse::ok("ok", None))
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("服务器迷路了")]
    NotFound,
    #[error("请求方法不支持")]
    MethodNotAllowed,
    #[error("数据库异常: {0}")]
    Database(#[from] DbErr),
    #[error("查询参数错误: {0}")]
    Query(#[from] QueryRejection),
    #[error("路径参数错误: {0}")]
    Path(#[from] PathRejection),
    #[error("Body参数错误: {0}")]
    Json(#[from] JsonRejection),
    #[error("参数验证失败: {0}")]
    Validation(String),
    #[error("密码Hash错误: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("JWT错误: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("未授权: {0}")]
    Unauthenticated(String),
    #[allow(dead_code)]
    #[error("{0}")]
    Biz(String),
    #[allow(dead_code)]
    #[error("{0}")]
    BizCode(ErrorCode),
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidRejection::Valid(error) => ApiError::Validation(error.to_string()),
            ValidRejection::Inner(error) => error,
        }
    }
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) | ApiError::BizCode(_) => StatusCode::OK,
            ApiError::Internal(_) | ApiError::Database(_) | ApiError::Bcrypt(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::JWT(_) | ApiError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = match self { 
            ApiError::BizCode(ec) => {
                ApiResponse::biz_err(ec)
            },
            err => {
                ApiResponse::<()>::err(err.to_string())
            }
        };
        let body = axum::Json(body);
        (status_code, body).into_response()
    }
}

impl From<ApiError> for Response {
    fn from(value: ApiError) -> Self {
        value.into_response()
    }
}

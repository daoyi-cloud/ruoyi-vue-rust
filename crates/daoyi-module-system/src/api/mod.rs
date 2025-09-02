use axum::Router;
use daoyi_common::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/api", Router::new())
}

// 错误示范（违反孤儿规则）
// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> axum::response::Response {
//         tracing::error!("{:?}", self);
//         (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
//     }
// }

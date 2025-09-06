mod admin;
mod app;

use axum::Router;
use daoyi_common::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api", admin::create_router())
        .nest("/app-api", app::create_router())
}

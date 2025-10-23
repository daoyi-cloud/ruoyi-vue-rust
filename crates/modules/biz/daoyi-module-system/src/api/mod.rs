mod admin;
mod app;

use axum::Router;
use daoyi_common::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api/system", admin::create_router())
        .nest("/app-api/system", app::create_router())
}

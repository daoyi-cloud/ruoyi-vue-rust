use axum::Router;
use daoyi_common::app::AppState;

pub mod auth;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/auth", auth::create_router())
}

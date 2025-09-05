mod auth;

use axum::Router;
use daoyi_common::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/api", Router::new().nest("/auth", auth::create_router()))
}

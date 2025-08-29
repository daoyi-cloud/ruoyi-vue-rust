use axum::Router;
use daoyi_common::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/demo", daoyi_module_demo::api::create_router())
}

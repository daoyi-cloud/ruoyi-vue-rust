use axum::Router;
use daoyi_common::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/demo", daoyi_module_demo::api::create_router())
        .nest("/system", daoyi_module_system::api::create_router())
        .nest("/infra", daoyi_module_infra::api::create_router())
}

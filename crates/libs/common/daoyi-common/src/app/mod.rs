pub mod auth;
pub mod common;
pub mod database;
pub mod latency;
pub mod logger;
mod middlewares;
pub mod redis_util;
pub mod server;

use crate::config;
use axum::Router;
pub use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting app server...");
    utils::id::init()?;
    redis_util::init_redis().await?;
    database::init_db().await?;
    let state = AppState::new();
    let server = server::Server::new(config::get().await.server());

    server.start(state, router).await
}

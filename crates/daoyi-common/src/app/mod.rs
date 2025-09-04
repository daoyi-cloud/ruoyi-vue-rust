pub mod auth;
pub mod common;
pub mod database;
pub mod enumeration;
pub mod error;
pub mod id;
pub mod json;
pub mod latency;
pub mod logger;
pub mod middleware;
pub mod path;
pub mod query;
pub mod redis;
pub use crate::app::redis as app_redis;
pub mod response;
pub mod serde;
pub mod server;
pub mod utils;
pub mod valid;
pub mod validation;

use crate::config;
use axum::Router;

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
    id::init()?;
    redis::init_redis().await?;
    database::init_db().await?;
    let state = AppState::new();
    let server = server::Server::new(config::get().server());

    server.start(state, router).await
}

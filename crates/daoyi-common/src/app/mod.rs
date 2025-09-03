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
mod redis;
pub mod response;
pub mod serde;
pub mod server;
pub mod utils;
pub mod valid;
pub mod validation;

use crate::config;
use axum::Router;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    id::init()?;
    tracing::info!("Starting app server...");
    redis::test_redis()?;
    let db = database::init().await?;
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());

    server.start(state, router).await
}

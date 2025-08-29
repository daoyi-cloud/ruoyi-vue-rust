mod api;

use daoyi_common::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}

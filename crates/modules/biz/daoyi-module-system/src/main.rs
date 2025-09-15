use daoyi_common::app;
use daoyi_module_system::api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}

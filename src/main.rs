mod api;

use daoyi_common::app::{self, openapi};
use daoyi_module_system::api::admin::auth::AuthApiDoc;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let doc = openapi::build_openapi_with(&[AuthApiDoc::openapi()]);
    openapi::register_openapi(doc);
    app::run(api::create_router()).await
}

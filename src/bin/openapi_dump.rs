use daoyi_common::app::openapi;
use daoyi_module_system::api::admin::auth::AuthApiDoc;
use utoipa::OpenApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = openapi::build_openapi_with(&[AuthApiDoc::openapi()]);
    println!("{}", serde_json::to_string_pretty(&doc)?);
    Ok(())
}

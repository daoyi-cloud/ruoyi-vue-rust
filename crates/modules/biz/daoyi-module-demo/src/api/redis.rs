use axum::{Router, routing};
use daoyi_common::app::{AppState, redis_util};
use daoyi_common_support::utils::errors::error::{ApiJsonResult, api_json_ok};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/current-test-key", routing::get(current_test_key))
}

async fn current_test_key() -> ApiJsonResult<String> {
    api_json_ok(redis_util::test_redis().await?)
}

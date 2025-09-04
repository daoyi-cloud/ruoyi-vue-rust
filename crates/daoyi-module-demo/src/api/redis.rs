use axum::{Router, routing};
use daoyi_common::app::{
    AppState, app_redis,
    error::{ApiJsonResult, api_json_ok},
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/current-test-key", routing::get(current_test_key))
}

async fn current_test_key() -> ApiJsonResult<String> {
    api_json_ok(app_redis::test_redis()?)
}

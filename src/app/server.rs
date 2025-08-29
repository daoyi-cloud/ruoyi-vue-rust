use crate::app::auth::Principal;
use crate::app::{AppState, latency::LatencyOnResponse};
use crate::config::ServerConfig;
use axum::extract::{ConnectInfo, DefaultBodyLimit};
use axum::{Router, extract::Request};
use bytesize::ByteSize;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.port())).await?;
        tracing::info!("Listening on {}", listener.local_addr()?);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let timeout = TimeoutLayer::new(Duration::from_secs(120));
        let body_limit = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .expose_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));
        let normalize_path = NormalizePathLayer::trim_trailing_slash();
        let trace = TraceLayer::new_for_http()
            .make_span_with(|result: &Request| {
                let method = result.method();
                let path = result.uri().path();
                let req_id = xid::new();
                let ext = result.extensions();
                // 尝试从 extensions 中获取客户端 IP
                let client_ip = ext
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|ConnectInfo(addr)| addr.ip().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                // 尝试从 extensions 中获取用户信息
                let user_info = ext
                    .get::<Principal>()
                    .map(|p| format!("user_id:{},user_name:{}", p.id, p.name))
                    .unwrap_or_else(|| "unknown".to_string());
                tracing::info_span!(
                    "api req ",
                    req_id = %req_id,
                    method = %method,
                    path = %path,
                    client_ip = %client_ip,
                    user_info = %user_info,
                )
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);

        Router::new()
            .merge(router)
            .layer(timeout)
            .layer(body_limit)
            .layer(trace)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
    }
}

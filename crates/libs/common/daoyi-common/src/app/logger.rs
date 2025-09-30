use crate::config;
use anyhow::anyhow;
use std::sync::OnceLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry, reload};

static RELOAD_HANDLE: OnceLock<reload::Handle<EnvFilter, Registry>> = OnceLock::new();
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let (filter, reload_handle) = reload::Layer::new(filter);

    RELOAD_HANDLE
        .set(reload_handle)
        .expect("Reload handle already set");

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .init();
}
pub async fn update_log_level() -> anyhow::Result<()> {
    let c = config::get().await;
    let level = c.server().log_level();
    let new_filter = EnvFilter::try_new(level)?;
    if let Some(handle) = RELOAD_HANDLE.get() {
        handle.reload(new_filter)?;
        Ok(())
    } else {
        Err(anyhow!("Logger not initialized"))
    }
}

use crate::app::logger;
use crate::config::{AppConfig, default_bool, get};
use anyhow::{Context, anyhow};
use config::{Config, FileFormat};
use nacos_sdk::api::config::{
    ConfigChangeListener, ConfigResponse, ConfigService, ConfigServiceBuilder,
};
use nacos_sdk::api::props::ClientProps;
use serde::Deserialize;
use std::sync::{Arc, LazyLock};
use tokio::sync::OnceCell;

static CONFIG: LazyLock<NacosConfig> =
    LazyLock::new(|| NacosConfig::load().expect("Failed to initialize nacos config"));

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NacosConfig {
    #[serde(default = "default_bool")]
    enable: bool,
    server_addr: String,
    namespace: String,
    app_name: String,
    group: String,
    auth_username: String,
    auth_password: String,
}

impl NacosConfig {
    pub fn enable(&self) -> bool {
        self.enable
    }
    pub fn server_addr(&self) -> &str {
        &self.server_addr
    }
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    pub fn group(&self) -> &str {
        &self.group
    }
    pub fn auth_username(&self) -> &str {
        &self.auth_username
    }
    pub fn auth_password(&self) -> &str {
        &self.auth_password
    }
}

impl NacosConfig {
    fn load() -> anyhow::Result<Self> {
        // 从启动参数中获取nacos.yaml的路径
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "resources".to_string());
        Config::builder()
            .add_source(
                config::File::with_name(format!("{}/nacos", config_path).as_ref())
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("NACOS")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow!("Failed to load config"))?
            .try_deserialize()
            .with_context(|| anyhow!("Failed to parse config"))
    }
}

fn get_nacos_config() -> &'static NacosConfig {
    &CONFIG
}

impl Into<ClientProps> for NacosConfig {
    fn into(self) -> ClientProps {
        ClientProps::new()
            .server_addr(self.server_addr())
            .namespace(self.namespace())
            .app_name(self.app_name())
            .auth_username(self.auth_username())
            .auth_password(self.auth_password())
    }
}

struct NacosConfigChangeListener;

impl ConfigChangeListener for NacosConfigChangeListener {
    fn notify(&self, config_resp: ConfigResponse) {
        tracing::info!("listen the config={:#?}", config_resp);
        tokio::spawn(async move {
            if let Err(e) = super::refresh().await {
                tracing::error!("Failed to update config: {:?}", e);
            }
            // 更新日志级别
            let _ = logger::update_log_level().await;
        });
    }
}
static CONFIG_SERVICE: OnceCell<ConfigService> = OnceCell::const_new();

async fn init_config_service() -> anyhow::Result<ConfigService> {
    let nacos_config = get_nacos_config();
    let config_service = ConfigServiceBuilder::new(nacos_config.to_owned().into())
        .enable_auth_plugin_http()
        .build()?;
    // add a listener
    let _listen = config_service
        .add_listener(
            String::from(nacos_config.app_name()),
            String::from(nacos_config.group()),
            Arc::new(NacosConfigChangeListener),
        )
        .await;
    match _listen {
        Ok(_) => tracing::info!("listening the config success"),
        Err(err) => tracing::info!("listen config error {:?}", err),
    }
    Ok(config_service)
}

async fn get_service() -> &'static ConfigService {
    CONFIG_SERVICE
        .get_or_init(|| async {
            init_config_service()
                .await
                .expect("Failed to initialize nacos config_service")
        })
        .await
}

pub async fn load_nacos_config() -> anyhow::Result<Option<ConfigResponse>> {
    // 请注意！一般情况下，应用下仅需一个 Config 客户端，而且需要长期持有直至应用停止。
    // 因为它内部会初始化与服务端的长链接，后续的数据交互及变更订阅，都是实时地通过长链接告知客户端的。
    let nacos_config = get_nacos_config();
    if !nacos_config.enable() {
        return Ok(None);
    }
    let config_service = get_service().await;
    let config_resp = config_service
        .get_config(
            String::from(nacos_config.app_name()),
            String::from(nacos_config.group()),
        )
        .await?;
    Ok(Some(config_resp))
}

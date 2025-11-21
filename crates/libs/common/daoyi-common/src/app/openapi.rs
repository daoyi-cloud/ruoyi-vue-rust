use axum::Router;
use daoyi_common_support::utils::web::response::{ApiJsonResponseBool, ApiJsonResponseString};
use std::sync::OnceLock;
use utoipa::{
    Modify, OpenApi,
    openapi::{
        Components,
        security::{HttpAuthScheme, HttpBuilder, SecurityRequirement, SecurityScheme},
    },
};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

use super::AppState;

/// OpenAPI 文档配置
#[derive(OpenApi)]
#[openapi(
    info(
        title = "若依 Rust 管理系统 API",
        version = "1.0.0",
        description = "基于 Axum + SeaORM + PostgreSQL 的企业级管理系统",
        contact(
            name = "Daoyi Cloud",
            email = "913573586@qq.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "开发环境"),
        (url = "https://api.example.com", description = "生产环境")
    ),
    components(
        schemas(ApiJsonResponseString, ApiJsonResponseBool)
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "认证管理"),
        (name = "system", description = "系统管理"),
        (name = "demo", description = "演示模块"),
        (name = "infra", description = "基础设施管理"),
    )
)]
pub struct ApiDoc;

static REGISTERED_OPENAPI: OnceLock<utoipa::openapi::OpenApi> = OnceLock::new();

/// 添加安全认证配置
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("请输入 JWT Token"))
                        .build(),
                ),
            );

            components.add_security_scheme(
                "tenant_id",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .description(Some("租户 ID，在请求头 tenant-id 中传递"))
                        .build(),
                ),
            );
        }

        openapi
            .security
            .get_or_insert_with(Vec::new)
            .push(SecurityRequirement::new("bearer_auth", std::iter::empty::<&str>()));
    }
}

/// 创建 OpenAPI 文档路由
pub fn create_openapi_router() -> Router<AppState> {
    let doc = REGISTERED_OPENAPI
        .get()
        .cloned()
        .unwrap_or_else(ApiDoc::openapi);

    Router::new()
        // Swagger UI - 标准的 Swagger 文档界面
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc.clone()))
        // Scalar - 现代化的 API 文档界面 (推荐)
        .merge(Scalar::with_url("/scalar", doc))
}

/// 注册外部模块生成的 OpenAPI 文档
pub fn register_openapi(doc: utoipa::openapi::OpenApi) {
    let _ = REGISTERED_OPENAPI.set(doc);
}

/// 构建包含附加模块信息的 OpenAPI 文档
pub fn build_openapi_with(additional: &[utoipa::openapi::OpenApi]) -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();
    for other in additional {
        merge_paths(&mut doc, other);
    }
    doc
}

/// 辅助函数：合并路径
#[allow(dead_code)]
fn merge_paths(base: &mut utoipa::openapi::OpenApi, other: &utoipa::openapi::OpenApi) {
    for (path, item) in &other.paths.paths {
        base.paths.paths.insert(path.clone(), item.clone());
    }

    if let Some(components) = &other.components {
        let base_components = base.components.get_or_insert_with(Components::default);

        for (name, schema) in &components.schemas {
            base_components
                .schemas
                .insert(name.clone(), schema.clone());
        }
    }
}

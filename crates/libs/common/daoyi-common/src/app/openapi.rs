use axum::Router;
use daoyi_common_support::utils::errors::error::ApiJsonResponse;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
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
        schemas(
            ApiJsonResponse<String>,
            ApiJsonResponse<bool>,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "认证管理"),
        (name = "user", description = "用户管理"),
        (name = "system", description = "系统管理"),
        (name = "demo", description = "演示模块"),
    )
)]
pub struct ApiDoc;

/// 添加安全认证配置
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
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
    }
}

/// 创建 OpenAPI 文档路由
pub fn create_openapi_router() -> Router<AppState> {
    Router::new()
        // Swagger UI - 标准的 Swagger 文档界面
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Scalar - 现代化的 API 文档界面 (推荐)
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
}

/// 合并多个 OpenAPI 文档
pub fn merge_openapi_docs() -> utoipa::openapi::OpenApi {
    let mut main_doc = ApiDoc::openapi();
    
    // 这里可以合并其他模块的 OpenAPI 文档
    // 例如: merge_paths(&mut main_doc, system_module::ApiDoc::openapi());
    
    main_doc
}

/// 辅助函数：合并路径
#[allow(dead_code)]
fn merge_paths(base: &mut utoipa::openapi::OpenApi, other: utoipa::openapi::OpenApi) {
    if let Some(paths) = other.paths {
        if let Some(base_paths) = &mut base.paths {
            for (path, item) in paths.paths {
                base_paths.paths.insert(path, item);
            }
        }
    }
    
    if let Some(components) = other.components {
        if let Some(base_components) = &mut base.components {
            if let Some(schemas) = components.schemas {
                for (name, schema) in schemas {
                    base_components.schemas.insert(name, schema);
                }
            }
        }
    }
}

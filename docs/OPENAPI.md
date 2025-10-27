# OpenAPI 文档集成指南

## 概述

本项目已集成 OpenAPI (Swagger) 文档功能，提供了两种现代化的 API 文档界面。

## 技术栈

- **utoipa**: Rust OpenAPI 文档生成库
- **utoipa-swagger-ui**: 传统 Swagger UI 界面
- **utoipa-scalar**: 现代化的 API 文档界面（推荐）

## 访问文档

启动项目后，可以通过以下地址访问 API 文档：

### Swagger UI（传统界面）
```
http://localhost:8080/swagger-ui
```

### Scalar（现代化界面，推荐）
```
http://localhost:8080/scalar
```

### OpenAPI JSON 规范
```
http://localhost:8080/api-docs/openapi.json
```

## 主要特性

✅ **自动生成文档** - 通过代码注解自动生成 OpenAPI 3.0 规范  
✅ **交互式测试** - 直接在浏览器中测试 API 接口  
✅ **JWT 认证支持** - 内置 Bearer Token 认证  
✅ **多租户支持** - 支持租户 ID 传递  
✅ **Schema 验证** - 自动生成请求/响应模型  
✅ **中文描述** - 完整的中文接口说明  

## 为 API 添加文档注解

### 1. 为数据模型添加 Schema

在 VO 结构体上添加 `ToSchema` derive：

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// 用户名
    #[schema(example = "admin")]
    pub username: String,
    
    /// 密码
    #[schema(example = "admin123")]
    pub password: String,
}
```

### 2. 为 API 端点添加文档

使用 `#[utoipa::path]` 宏为 API 端点添加文档：

```rust
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    summary = "用户登录",
    description = "使用账号密码登录系统",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = ApiJsonResponse<LoginResponse>),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "账号或密码错误"),
    ),
    security(
        ("bearer_auth" = [])  // 需要认证的接口添加此行
    )
)]
#[debug_handler]
async fn login(
    ValidJson(params): ValidJson<LoginRequest>,
) -> ApiJsonResult<LoginResponse> {
    // ... 实现代码
}
```

### 3. 创建模块级别的 OpenAPI 文档

为每个模块创建 OpenAPI 文档配置：

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        login,
        logout,
        // ... 其他 API
    ),
    components(
        schemas(
            LoginRequest,
            LoginResponse,
            // ... 其他模型
        )
    ),
    tags(
        (name = "auth", description = "认证管理 API")
    )
)]
pub struct AuthApiDoc;
```

## 认证配置

OpenAPI 文档页面无需认证即可访问。配置位于 `resources/application.yaml`：

```yaml
auth:
  ignore_urls:
    - /swagger-ui
    - /swagger-ui/*
    - /scalar
    - /scalar/*
    - /api-docs/*

tenant:
  ignore_urls:
    - /swagger-ui
    - /swagger-ui/*
    - /scalar
    - /scalar/*
    - /api-docs/*
```

## 使用 JWT 认证测试 API

在 Swagger UI 或 Scalar 界面中：

1. 点击 **Authorize** 或 **🔒** 按钮
2. 在 `bearer_auth` 字段中输入 JWT Token
3. 点击 **Authorize** 确认
4. 现在可以测试需要认证的 API 了

Token 格式：
```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## 常见的 Schema 注解

```rust
/// 基础示例
#[schema(example = "示例值")]
pub field: String,

/// 字符串类型重写（用于 DateTime 等）
#[schema(value_type = String, example = "2024-12-31 23:59:59")]
pub created_at: DateTime,

/// 可选字段
#[schema(nullable = true)]
pub optional_field: Option<String>,

/// 数组
#[schema(example = json!(["admin", "user"]))]
pub roles: Vec<String>,

/// 范围限制
#[schema(minimum = 0, maximum = 100)]
pub age: i32,
```

## 最佳实践

1. **为所有 VO 添加 `ToSchema`** - 确保文档完整性
2. **添加示例值** - 使用 `#[schema(example = "...")]` 提供示例
3. **详细的描述** - 在三斜杠注释中提供清晰的说明
4. **合理的 Tag 分组** - 按业务模块组织 API
5. **统一的响应格式** - 使用 `ApiJsonResponse<T>` 包装响应

## 项目结构

```
crates/
├── libs/common/daoyi-common/
│   └── src/app/
│       └── openapi.rs              # OpenAPI 主配置
└── modules/biz/
    └── daoyi-module-system/
        └── src/
            ├── api/admin/
            │   └── auth.rs          # API 文档注解
            └── vo/auth/
                ├── *.rs             # VO Schema 定义
```

## 故障排查

### 文档页面无法访问

1. 确认服务已启动在正确端口 (8080)
2. 检查防火墙设置
3. 查看控制台是否有错误日志

### API 没有显示在文档中

1. 检查是否添加了 `#[utoipa::path]` 注解
2. 确认 path 函数已在 `OpenApi` 的 `paths()` 中注册
3. 检查 schema 是否在 `components/schemas()` 中注册

### Schema 显示不正确

1. 确认 VO 实现了 `ToSchema`
2. 检查 `#[serde(rename_all = "camelCase")]` 是否正确
3. 对于复杂类型，使用 `#[schema(value_type = ...)]` 重写

## 参考资源

- [utoipa 官方文档](https://github.com/juhaku/utoipa)
- [OpenAPI 3.0 规范](https://swagger.io/specification/)
- [Scalar 文档](https://github.com/ScalaConsultants/scalar)

## 更新日志

- **2024-10-23**: 初始集成 OpenAPI 文档功能
  - 添加 Swagger UI 支持
  - 添加 Scalar UI 支持
  - 完成认证模块文档注解
  - 配置白名单访问

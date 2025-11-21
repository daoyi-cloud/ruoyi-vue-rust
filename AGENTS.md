# Repository Guidelines

## 项目结构与模块划分
工作区根目录包含 `Cargo.toml`、`Cargo.lock` 与入口 `src/main.rs`，该二进制通过 `api` 模块挂载 Axum 路由并在 `target/` 中输出构建产物。共享工具与支持库位于 `crates/libs/common/*/*`，实体定义集中在 `crates/libs/entities/*/*`。实际业务分为 system、infra、demo 等 crate，统一放在 `crates/modules/biz/daoyi-module-*` 内，若需要新增 API 适配层，请放在 `crates/modules/apis` 并引入模块公共接口。文档、配置与 SQL 脚本分别位于 `docs/`、`resources/` 与 `sql/`；示例配置应以 `*.example` 结尾。每个 crate 自带 `tests/` 或 `#[cfg(test)]` 模块，便于按包独立构建与调试，也方便 `cargo test -p <crate>` 进行增量验证。

## 构建、测试与开发命令
- `cargo fmt && cargo clippy --workspace --all-targets`：运行格式化与静态检查，确保所有 crate 一致，CI 也将执行同样的步骤。
- `cargo build --workspace`：编译工作区全部库与可执行文件，验证依赖锁定与 feature 组合。
- `RUST_LOG=DEBUG cargo run --package ruoyi-vue-rust`：启动主服务并输出 SeaORM SQL，方便排查。
- `cargo test --workspace -- --nocapture`：执行单元与集成测试，同时打印日志。
- `sea-orm-cli generate entity -s dev --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --date-time-crate chrono -o ./crates/daoyi-module-demo/src/entity`：在表结构变更后生成实体；根据模块调整 schema 与输出路径，并记得同步 `Cargo.toml` 中的实体依赖。

## 代码风格与命名约定
统一使用 `rustfmt` 默认的 4 空格缩进与尾随逗号，模块与文件目录遵循 `snake_case`，类型与结构体保持 `PascalCase`。API DTO 通过 `serde(rename_all = "camelCase")` 与前端保持一致，字段命名避免缩写。优先使用 `pub(crate)` 控制可见性，控制器层保持薄，逻辑放入 `daoyi-common` 或业务服务中，仅在必要处添加简短注释说明。涉及 SQL 枚举或常量时，将字符串常量集中在 `constants.rs` 或模块级 `const` 以便复用。

## 测试指引
在对应 crate 内创建 `tests/` 目录或内联 `mod tests`，测试函数命名建议为 `should_<action>_<result>()`，必要时使用 `#[tokio::test]` 驱动异步流程。HTTP 集成测试应放在相关业务模块中，并模拟真实路由与权限。涉及数据库的测试使用 SeaORM fixture 预置数据，可通过 `RUST_LOG=DEBUG cargo test -p daoyi-module-system` 观察 SQL 并确保事务回滚。提交前覆盖校验、仓储与控制器层，避免未测路径，并在 PR 中提及任何测试跳过的原因。

## 提交与 Pull Request 规范
沿用 Conventional Commits，例如 `feat(system): 调整系统API路由前缀`、`refactor(config): ...`、`chore(deps): ...`，必要时追加作用域（如 `feat(auth)!:` 表示破坏性更新）。单次提交聚焦一个模块，可使用中英文简洁描述并在结尾引用 Issue ID。PR 需引用对应 Issue，说明功能影响、潜在破坏性调整，并附上截图或 `curl` 结果。附带 `cargo test --workspace` 的运行结论，同时列出需要 reviewer 执行的 schema、配置或环境变量调整，最后更新相关文档或样例配置。

## 配置与环境提示
所有 `.env`、YAML 或样例配置放在 `resources/`，严禁提交真实密钥；需要新环境变量时请提供 `.example` 文件说明默认值。变更日志级别或追踪行为时，请在 PR 中注明必须设置的变量（如 `RUST_LOG=DEBUG`、`NAOCS_SERVER_ADDR`）。数据库序列或脚本应添加到 `sql/`，并在 PR 描述中标注文件名，确保 reviewer 能先执行再启动服务；若脚本依赖顺序，务必在 `docs/` 中额外补充执行说明。

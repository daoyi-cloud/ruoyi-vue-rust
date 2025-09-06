- Git 全局设置:

    ```shell
    git config --global user.name "Gemiman.2#"
    git config --global user.email "913573586@qq.com"
    ```

- 创建 git 仓库:

    ```shell
    cargo new ruoyi-vue-rust
    cd ruoyi-vue-rust 
    git add .
    git commit -m "first commit"
    git remote add origin git@github.com:daoyi-cloud/ruoyi-vue-rust.git
    git branch -M main
    git push -u origin main
    ```

- 安装 sea-orm-cli
    ```shell
    cargo install sea-orm-cli
    ```
  - 生成 demo entity
      ```shell
      sea-orm-cli generate entity -s demo --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --date-time-crate chrono -o ./src/entity
      ```

    - 输出sql日志，配置环境变量：

        ```text
        RUST_LOG=DEBUG
        ```
  
      - 生成 dev entity
        ```shell
          sea-orm-cli generate entity -s dev --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --date-time-crate chrono -o ./crates/daoyi-module-demo/src/entity
        ```
- 序列
```sql
-- 将序列与表的 id 字段关联
ALTER TABLE system_oauth2_refresh_token
ALTER COLUMN id SET DEFAULT nextval('system_oauth2_refresh_token_seq');
    
-- 设置序列由表拥有
ALTER SEQUENCE system_oauth2_refresh_token_seq
OWNED BY system_oauth2_refresh_token.id;
```
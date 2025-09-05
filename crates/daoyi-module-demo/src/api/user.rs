use crate::entity::{prelude::*, sys_user, sys_user::ActiveModel};
use anyhow::Context;
use axum::{Router, debug_handler, routing};
use daoyi_common::app::{
    AppState,
    common::{Page, PaginationParams},
    database,
    enumeration::Gender,
    errors::error::{ApiError, ApiJsonResult, api_empty_ok, api_json_ok},
    path::Path,
    utils::encode_password,
    valid::{ValidJson, ValidQuery},
    validation::is_mobile_phone,
};
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryOrder, QueryTrait, prelude::*};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(find_page))
        .route("/", routing::post(create))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::delete(delete))
        .route("/all", routing::get(query_users))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserParams {
    keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct UserParams {
    #[validate(length(min = 1, max = 16, message = "姓名长度1-16"))]
    pub name: String,
    pub gender: Gender,
    #[validate(length(min = 1, max = 16, message = "账号长度1-16"))]
    pub account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度6-16"))]
    pub password: String,
    #[validate(custom(function = "is_mobile_phone"))]
    pub mobile_phone: String,
    pub birthday: Date,
    #[serde(default)]
    pub enabled: bool,
}

#[debug_handler]
async fn create(ValidJson(params): ValidJson<UserParams>) -> ApiJsonResult<sys_user::Model> {
    let mut active_model = params.into_active_model();
    active_model.password =
        ActiveValue::Set(encode_password(&active_model.password.take().unwrap())?);
    let model = active_model.insert(database::get()?).await?;
    api_json_ok(model)
}

#[debug_handler]
async fn update(
    Path(id): Path<i64>,
    ValidJson(params): ValidJson<UserParams>,
) -> ApiJsonResult<sys_user::Model> {
    let db = database::get()?;
    let existed_user = SysUser::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待修改用户不存在")))?;
    let old_password = existed_user.password.clone();
    let password = params.password.clone();
    let mut exist_active_model = existed_user.into_active_model();
    let mut active_model = params.into_active_model();
    exist_active_model.clone_from(&active_model);
    exist_active_model.id = ActiveValue::Unchanged(id);
    if password.is_empty() {
        exist_active_model.password = ActiveValue::Unchanged(old_password);
    } else {
        exist_active_model.password =
            ActiveValue::Set(encode_password(&active_model.password.take().unwrap())?);
    }
    let result = exist_active_model.update(db).await?;
    api_json_ok(result)
}

#[debug_handler]
async fn delete(Path(id): Path<i64>) -> ApiJsonResult<()> {
    let db = database::get()?;
    let existed_user = SysUser::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待删除用户不存在")))?;
    let result = existed_user.delete(db).await?;
    tracing::info!(
        "Delete user: {}, affected rows: {}",
        id,
        result.rows_affected
    );
    api_empty_ok()
}

#[debug_handler]
async fn find_page(
    ValidQuery(QueryUserParams {
        keyword,
        pagination,
    }): ValidQuery<QueryUserParams>,
) -> ApiJsonResult<Page<sys_user::Model>> {
    let db = database::get()?;
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(db, pagination.size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = Page::from_pagination(pagination, total, items);
    api_json_ok(page)
}

#[debug_handler]
#[tracing::instrument(name = "query_users", skip_all, fields(pay_method = "alipay"))]
async fn query_users() -> ApiJsonResult<Vec<sys_user::Model>> {
    tracing::warn!("假装出现错误了");
    // 假装超时了
    // tokio::time::sleep(std::time::Duration::from_secs(225)).await;
    let users = SysUser::find()
        .filter(sys_user::Column::Gender.eq("male"))
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("male"))
                .add(sys_user::Column::Name.starts_with("张"))
                .add(
                    Condition::any()
                        .add(sys_user::Column::Name.contains("张"))
                        .add(sys_user::Column::Name.contains("王"))
                        .add(sys_user::Column::Enabled.eq(true)),
                ),
        )
        .all(database::get()?)
        .await
        .context("查询用户列表失败")?;
    api_json_ok(users)
}

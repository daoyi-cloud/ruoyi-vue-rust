use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::CommonStatusEnum;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{POST_NOT_ENABLE, POST_NOT_FOUND};
use daoyi_entities_system::entity::prelude::SystemPost;
use daoyi_entities_system::entity::system_post;
use sea_orm::*;

pub struct PostService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(PostService);
impl PostService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemPost> {
        SystemPost::find()
            .filter(system_post::Column::TenantId.eq(self.tenant_id()))
            .filter(system_post::Column::Deleted.eq(0))
    }
}

impl PostService {
    pub async fn validate_post_list(&self, post_ids: Vec<i64>) -> ApiResult<()> {
        if post_ids.is_empty() {
            return Ok(());
        }
        let len = post_ids.len();
        let post_list = self
            .base_query()
            .filter(system_post::Column::Id.is_in(post_ids))
            .all(database::get()?)
            .await?;
        if post_list.len() != len {
            return Err(ApiError::BizCode(POST_NOT_FOUND));
        }
        for post in post_list {
            if CommonStatusEnum::is_disable(post.status) {
                return Err(ApiError::BizCodeWithArgs(POST_NOT_ENABLE, vec![post.name]));
            }
        }
        Ok(())
    }
}

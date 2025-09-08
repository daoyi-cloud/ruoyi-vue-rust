use crate::support::tenant::TenantContextHolder;
use crate::utils::errors::error::ApiError;
use sea_orm::ActiveModelTrait;

/// 通用字段更新trait
pub trait CommonFieldsUpdater {
    /// 设置创建字段
    fn set_created_fields(&mut self, creator: Option<String>);

    /// 设置更新字段
    fn set_updated_fields(&mut self, updater: Option<String>);

    /// 设置删除状态
    fn set_deleted(&mut self, deleted: i32);

    /// 设置租户ID
    fn set_tenant_id(&mut self, tenant_id: i64);
}
/// 通用的创建操作函数
pub async fn create_with_common_fields<T>(
    mut active_model: T,
    creator: Option<String>,
    tenant_holder: &TenantContextHolder,
) -> Result<T, ApiError>
where
    T: ActiveModelTrait + CommonFieldsUpdater,
{
    // 设置通用字段
    active_model.set_created_fields(creator.clone());
    active_model.set_updated_fields(creator);
    active_model.set_tenant_id(tenant_holder.tenant_id());
    active_model.set_deleted(0); // 默认未删除

    Ok(active_model)
}

/// 通用的更新操作函数
pub async fn update_with_common_fields<T>(
    mut active_model: T,
    updater: Option<String>,
) -> Result<T, ApiError>
where
    T: ActiveModelTrait + CommonFieldsUpdater,
{
    // 设置更新字段
    active_model.set_updated_fields(updater);

    Ok(active_model)
}

/// 通用的软删除操作函数
pub async fn soft_delete_with_common_fields<T>(
    mut active_model: T,
    deleter: Option<String>,
) -> Result<T, ApiError>
where
    T: ActiveModelTrait + CommonFieldsUpdater,
{
    // 设置删除字段
    active_model.set_deleted(1); // 标记为已删除
    active_model.set_updated_fields(deleter);

    Ok(active_model)
}

#[macro_export]
macro_rules! impl_common_fields_updater {
    ($entity:ty) => {
        impl CommonFieldsUpdater for $entity {
            fn set_created_fields(&mut self, creator: Option<String>) {
                self.creator = ActiveValue::Set(creator);
                self.create_time = ActiveValue::Set(chrono::Local::now().naive_local());
            }

            fn set_updated_fields(&mut self, updater: Option<String>) {
                self.updater = ActiveValue::Set(updater);
                self.update_time = ActiveValue::Set(chrono::Local::now().naive_local());
            }

            fn set_deleted(&mut self, deleted: i32) {
                self.deleted = ActiveValue::Set(deleted);
            }

            fn set_tenant_id(&mut self, tenant_id: i64) {
                self.tenant_id = ActiveValue::Set(tenant_id);
            }
        }
    };
}

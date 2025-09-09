use crate::app::database;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_infra::entity::prelude::InfraConfig;
use sea_orm::EntityTrait;

pub struct ConfigApi;

impl ConfigApi {
    pub async fn get_config_value_by_key(&self, key: &str) -> ApiResult<Option<String>> {
        let option = InfraConfig::find().one(database::get()?).await?;
        Ok(option.map(|x| x.value))
    }
}

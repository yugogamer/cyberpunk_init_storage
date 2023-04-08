use crate::utils::{config::Config, errors::AppErrors};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: i32,
    pub owner_id: i32,
    pub original_name: String,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAsset {
    pub original_name: String,
}

#[async_trait]
pub trait AssetStore {
    async fn new_asset(&self, new_asset: NewAsset) -> Result<(), AppErrors>;
    async fn get_asset(&self, id: i32) -> Result<Asset, AppErrors>;
    async fn get_asset_file(&self, id: i32) -> Result<(), AppErrors>;
}

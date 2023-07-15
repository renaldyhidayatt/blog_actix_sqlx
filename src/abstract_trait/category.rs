use anyhow::Result;
use std::sync::Arc;

use async_trait::async_trait;

use uuid::Uuid;

use crate::{domain::CategoryResponse, models::CategoryModel};

pub type DynCategoryRepository = Arc<dyn CategoryRepositoryTrait + Send + Sync>;
pub type DynCategoryService = Arc<dyn CategoryServiceTrait + Send + Sync>;

#[async_trait]
pub trait CategoryRepositoryTrait {
    async fn get_categories(&self) -> Result<Vec<CategoryModel>>;
    async fn get_category(&self, id: Uuid) -> Result<Option<CategoryModel>>;
    async fn create_category(&self, name: &str) -> Result<CategoryModel>;
    async fn update_category(&self, id: Uuid, name: &str) -> Result<Option<CategoryModel>>;
    async fn delete_category(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait CategoryServiceTrait {
    async fn get_categories(&self) -> Result<Vec<CategoryResponse>>;
    async fn get_category(&self, id: Uuid) -> Result<Option<CategoryResponse>>;
    async fn create_category(&self, name: &str) -> Result<CategoryResponse>;
    async fn update_category(&self, id: Uuid, name: &str) -> Result<Option<CategoryResponse>>;
    async fn delete_category(&self, id: Uuid) -> Result<()>;
}

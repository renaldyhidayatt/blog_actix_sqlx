use crate::{
    abstract_trait::{CategoryServiceTrait, DynCategoryRepository},
    domain::CategoryResponse,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct CategoryService {
    repository: DynCategoryRepository,
}

impl CategoryService {
    pub fn new(repository: DynCategoryRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
    async fn get_categories(&self) -> Result<Vec<CategoryResponse>> {
        let categories = self.repository.get_categories().await?;

        let category_responses: Vec<CategoryResponse> = categories
            .into_iter()
            .map(|category| CategoryResponse::from(category))
            .collect();

        Ok(category_responses)
    }

    async fn get_category(&self, id: i32) -> Result<Option<CategoryResponse>> {
        let category = self.repository.get_category(id).await?;

        match category {
            Some(category) => {
                let category_response = CategoryResponse::from(category);
                Ok(Some(category_response))
            }
            None => Ok(None),
        }
    }

    async fn create_category(&self, name: &str) -> Result<CategoryResponse> {
        let category = self.repository.create_category(name).await?;

        let category_response = CategoryResponse::from(category);
        Ok(category_response)
    }

    async fn update_category(&self, id: i32, name: &str) -> Result<Option<CategoryResponse>> {
        let category = self.repository.update_category(id, name).await?;

        match category {
            Some(category) => {
                let category_response = CategoryResponse::from(category);
                Ok(Some(category_response))
            }
            None => Ok(None),
        }
    }

    async fn delete_category(&self, id: i32) -> Result<()> {
        self.repository.delete_category(id).await?;

        Ok(())
    }
}

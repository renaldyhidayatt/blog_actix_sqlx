use crate::models::CategoryModel;
use chrono::DateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub createdAt: Option<DateTime<chrono::Utc>>,
    pub updatedAt: Option<DateTime<chrono::Utc>>,
}

impl From<CategoryModel> for CategoryResponse {
    fn from(category: CategoryModel) -> Self {
        CategoryResponse {
            id: category.id,
            name: category.name,
            createdAt: category.created_at,
            updatedAt: category.updated_at,
        }
    }
}

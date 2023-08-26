use crate::abstract_trait::CategoryRepositoryTrait;
use crate::config::ConnectionPool;
use crate::models::CategoryModel;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

pub struct CategoryRepository {
    db_pool: ConnectionPool,
}

impl CategoryRepository {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CategoryRepositoryTrait for CategoryRepository {
    async fn get_categories(&self) -> Result<Vec<CategoryModel>> {
        let categories =
            sqlx::query_as::<_, CategoryModel>("SELECT * FROM categories ORDER BY id ASC")
                .fetch_all(&self.db_pool)
                .await?;

        Ok(categories)
    }

    async fn get_category(&self, id: i32) -> Result<Option<CategoryModel>> {
        let category = sqlx::query_as::<_, CategoryModel>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(category)
    }

    async fn create_category(&self, name: &str) -> Result<CategoryModel> {
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let category = sqlx::query_as::<_, CategoryModel>(
            "INSERT INTO categories (name, created_at, updated_at) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(created_at)
        .bind(updated_at)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(category)
    }

    async fn update_category(&self, id: i32, name: &str) -> Result<Option<CategoryModel>> {
        let updated_at = Utc::now();
        let category = sqlx::query_as::<_, CategoryModel>(
            "UPDATE categories SET name=$2, updated_at=$3 WHERE id=$1 RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(updated_at)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(category)
    }

    async fn delete_category(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM categories WHERE id = $1", id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}

use anyhow::{Result, Ok};
use crate::models::UserModel;
use crate::{abstract_trait::UserRepositoryTrait, config::ConnectionPool};
use async_trait::async_trait;
use chrono::Utc;
use sqlx:: Row;
use uuid::Uuid;

pub struct UserRepository {
    pub db_pool: ConnectionPool,
}

impl UserRepository {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_email_exists(&self, email: &str) -> Result<bool> {
        let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await?
            .get(0);
        Ok(exists)
    }

    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserModel> {
        let created_at = Utc::now();
        let updated_at = Utc::now();
        
        let query_result = sqlx::query_as::<_, UserModel>(
        
            "INSERT INTO users (firstname, lastname, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
           
        )
        .bind(firstname)
        .bind(lastname)
        .bind(email)
        .bind(password)
        .bind(created_at)
        .bind(updated_at) 
        .fetch_one(&self.db_pool)
        .await?;
        Ok(query_result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>> {
        let query_result =
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE email = $1")
            .bind(email)
                .fetch_optional(&self.db_pool)
                .await?;
        Ok(query_result)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<UserModel>> {
        let query_result = sqlx::query_as::<_, UserModel>( "SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(query_result)
    }

    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> Result<Option<UserModel>> {
        let query_result = sqlx::query_as::<_, UserModel>(
          
            "UPDATE users SET firstname = $1, lastname = $2, password = $3 WHERE email = $4 RETURNING *",
        )
        .bind(firstname)
        .bind(lastname)
        .bind(email)
        .bind(password)
        .fetch_optional(&self.db_pool)
        .await?;
        Ok(query_result)
    }

    async fn delete_user(&self, email: &str) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&self.db_pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

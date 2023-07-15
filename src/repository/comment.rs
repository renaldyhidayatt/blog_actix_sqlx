use anyhow::{Ok, Result};
use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::abstract_trait::CommentRepositoryTrait;
use crate::models::CommentModel;

pub struct CommentRepository {
    db_pool: sqlx::PgPool,
}

impl CommentRepository {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        CommentRepository { db_pool }
    }
}

#[async_trait]
impl CommentRepositoryTrait for CommentRepository {
    async fn get_comments(&self) -> Result<Vec<CommentModel>> {
        let comments = query_as::<_, CommentModel>("SELECT * FROM comments ORDER BY id DESC")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(comments)
    }

    async fn get_comment(&self, id: Uuid) -> Result<Option<CommentModel>> {
        let comment = query_as::<_, CommentModel>("SELECT * FROM comments WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(comment)
    }

    async fn create_comment(
        &self,
        id_post_comment: Uuid,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<CommentModel> {
        let comment = query_as::<_, CommentModel>(
            "INSERT INTO comments (id_post_comment, user_name_comment, comment) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(id_post_comment)
        .bind(user_name_comment)
        .bind(comment)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(comment)
    }

    async fn update_comment(
        &self,
        id: Uuid,
        id_post_comment: Uuid,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<Option<CommentModel>> {
        let comment = query_as::<_, CommentModel>(
            "UPDATE comments SET id_post_comment = $2, user_name_comment = $3, comment = $4 WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(id_post_comment)
        .bind(user_name_comment)
        .bind(comment)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(comment)
    }

    async fn delete_comment(&self, id: Uuid) -> Result<()> {
        query("DELETE FROM comments WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}

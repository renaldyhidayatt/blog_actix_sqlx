use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::abstract_trait::PostsRepositoryTrait;
use crate::config::ConnectionPool;
use crate::models::{PostModel, PostRelationModel};

pub struct PostsRepository {
    pool: ConnectionPool,
}

impl PostsRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        PostsRepository { pool }
    }
}

#[async_trait]
impl PostsRepositoryTrait for PostsRepository {
    async fn get_all_posts(&self) -> Result<Vec<PostModel>> {
        let posts = sqlx::query_as::<_, PostModel>(r#"SELECT * FROM posts ORDER BY id DESC"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(posts)
    }

    async fn get_post(&self, post_id: Uuid) -> Result<Option<PostModel>> {
        let post = sqlx::query_as::<_, PostModel>(
            r#"SELECT id, title, body, category_id, user_id, user_name FROM posts WHERE id = $1"#,
        )
        .bind(post_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn get_post_relation(&self, post_id: Uuid) -> Result<Vec<PostRelationModel>> {
        let post_relations = sqlx::query_as::<_, PostRelationModel>(
            r#"
            SELECT
                posts.id AS post_id,
                posts.title,
                comments.id AS comment_id,
                comments.id_post_comment,
                comments.user_name_comment,
                comments.comment
            FROM
                comments
            JOIN
                posts ON posts.id = comments.id_post_comment
            WHERE
                posts.id = $1
            "#,
        )
        .bind(post_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(post_relations)
    }

    async fn create_post(
        &self,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<PostModel> {
        let post = sqlx::query_as::<_, PostModel>(
            r#"
            INSERT INTO posts (title, body, category_id, user_id, user_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(body)
        .bind(category_id)
        .bind(user_id)
        .bind(user_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn update_post(
        &self,
        post_id: Uuid,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<Option<PostModel>> {
        let post = sqlx::query_as::<_, PostModel>(
            r#"
            UPDATE posts
            SET title = $2, body = $3, category_id = $4, user_id = $5, user_name = $6
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(post_id)
        .bind(title)
        .bind(body)
        .bind(category_id)
        .bind(user_id)
        .bind(user_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn delete_post(&self, post_id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM posts WHERE id = $1"#)
            .bind(post_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

use std::sync::Arc;

use crate::{domain::CommentResponse, models::CommentModel};
use anyhow::Result;
use async_trait::async_trait;

pub type DynCommentRepository = Arc<dyn CommentRepositoryTrait + Send + Sync>;
pub type DynCommentService = Arc<dyn CommentServiceTrait + Send + Sync>;

#[async_trait]
pub trait CommentRepositoryTrait {
    async fn get_comments(&self) -> Result<Vec<CommentModel>>;
    async fn get_comment(&self, id: i32) -> Result<Option<CommentModel>>;
    async fn create_comment(
        &self,
        id_post_comment: i32,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<CommentModel>;

    async fn update_comment(
        &self,
        id: i32,
        id_post_comment: i32,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<Option<CommentModel>>;

    async fn delete_comment(&self, id: i32) -> Result<()>;
}

#[async_trait]
pub trait CommentServiceTrait {
    async fn get_comments(&self) -> Result<Vec<CommentResponse>>;
    async fn get_comment(&self, id: i32) -> Result<Option<CommentResponse>>;
    async fn create_comment(
        &self,
        id_post_comment: i32,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<CommentResponse>;

    async fn update_comment(
        &self,
        id: i32,
        id_post_comment: i32,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<Option<CommentResponse>>;

    async fn delete_comment(&self, id: i32) -> Result<()>;
}

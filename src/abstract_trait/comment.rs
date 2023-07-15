use std::sync::Arc;

use crate::{models::CommentModel, domain::CommentResponse};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub type DynCommentRepository = Arc<dyn CommentRepositoryTrait + Send + Sync>;
pub type DynCommentsService = Arc<dyn CommentServiceTrait + Send + Sync>;

#[async_trait]
pub trait CommentRepositoryTrait {
    async fn get_comments(&self) -> Result<Vec<CommentModel>>;
    async fn get_comment(&self, id: Uuid) -> Result<Option<CommentModel>>;
    async fn create_comment(
        &self,
        id_post_comment: Uuid,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<CommentModel>;

    async fn update_comment(
        &self,
        id: Uuid,
        id_post_comment: Uuid,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<Option<CommentModel>>;

    async fn delete_comment(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait CommentServiceTrait {
    async fn get_comments(&self) -> Result<Vec<CommentResponse>>;
    async fn get_comment(&self, id: Uuid) -> Result<Option<CommentResponse>>;
    async fn create_comment(
        &self,
        id_post_comment: Uuid,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<CommentResponse>;
    async fn update_comment(
        &self,
        id: Uuid,
        id_post_comment: Uuid,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<Option<CommentResponse>>;
    async fn delete_comment(&self, id: Uuid) -> Result<()>;
}

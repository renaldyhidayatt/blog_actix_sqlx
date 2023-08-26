use crate::abstract_trait::{CommentServiceTrait, DynCommentRepository};
use crate::domain::CommentResponse;
use anyhow::Result;
use async_trait::async_trait;

pub struct CommentService {
    repository: DynCommentRepository,
}

impl CommentService {
    pub fn new(repository: DynCommentRepository) -> Self {
        CommentService { repository }
    }
}

#[async_trait]
impl CommentServiceTrait for CommentService {
    async fn get_comments(&self) -> Result<Vec<CommentResponse>> {
        let comments = self.repository.get_comments().await?;
        let comment_responses: Vec<CommentResponse> =
            comments.into_iter().map(|comment| comment.into()).collect();
        Ok(comment_responses)
    }

    async fn get_comment(&self, id: i32) -> Result<Option<CommentResponse>> {
        if let Some(comment) = self.repository.get_comment(id).await? {
            let comment_response: CommentResponse = comment.into();
            Ok(Some(comment_response))
        } else {
            Ok(None)
        }
    }

    async fn create_comment(
        &self,
        id_post_comment: i32,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<CommentResponse> {
        let created_comment = self
            .repository
            .create_comment(id_post_comment, user_name_comment, comment)
            .await?;
        let comment_response: CommentResponse = created_comment.into();
        Ok(comment_response)
    }

    async fn update_comment(
        &self,
        id: i32,
        id_post_comment: i32,
        user_name_comment: &str,
        comment: &str,
    ) -> Result<Option<CommentResponse>> {
        if let Some(updated_comment) = self
            .repository
            .update_comment(id, id_post_comment, user_name_comment, comment)
            .await?
        {
            let comment_response: CommentResponse = updated_comment.into();
            Ok(Some(comment_response))
        } else {
            Ok(None)
        }
    }

    async fn delete_comment(&self, id: i32) -> Result<()> {
        self.repository.delete_comment(id).await?;
        Ok(())
    }
}

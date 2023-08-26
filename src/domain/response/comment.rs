use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::models::CommentModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<chrono::Utc>>,
}

impl From<CommentModel> for CommentResponse {
    fn from(comment: CommentModel) -> Self {
        CommentResponse {
            id: comment.id,
            id_post_comment: comment.id_post_comment,
            user_name_comment: comment.user_name_comment,
            comment: comment.comment,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    }
}

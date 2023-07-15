use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub id_post_comment: Uuid,
    pub user_name_comment: String,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub id_post_comment: Uuid,
    pub user_name_comment: String,
    pub comment: String,
}

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct CommentModel {
    pub id: Uuid,
    pub id_post_comment: Uuid,
    pub user_name_comment: String,
    pub comment: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

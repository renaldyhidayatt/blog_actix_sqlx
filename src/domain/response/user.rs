use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::UserModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<chrono::Utc>>,
}

impl From<UserModel> for UserResponse {
    fn from(user: UserModel) -> Self {
        UserResponse {
            id: user.id,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

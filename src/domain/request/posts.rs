use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub body: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}

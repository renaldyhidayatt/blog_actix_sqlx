use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PostModel {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostRelationModel {
    pub post_id: Uuid,
    pub title: String,
    pub comment_id: i32,
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}

use crate::models::{PostModel, PostRelationModel};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}

impl From<PostModel> for PostResponse {
    fn from(post: PostModel) -> Self {
        PostResponse {
            id: post.id,
            title: post.title,
            body: post.body,
            category_id: post.category_id,
            user_id: post.user_id,
            user_name: post.user_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PostRelationResponse {
    pub post_id: Uuid,
    pub title: String,
    pub comment_id: i32,
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}

impl From<PostRelationModel> for PostRelationResponse {
    fn from(post_relation: PostRelationModel) -> Self {
        PostRelationResponse {
            post_id: post_relation.post_id,
            title: post_relation.title,
            comment_id: post_relation.comment_id,
            id_post_comment: post_relation.id_post_comment,
            user_name_comment: post_relation.user_name_comment,
            comment: post_relation.comment,
        }
    }
}

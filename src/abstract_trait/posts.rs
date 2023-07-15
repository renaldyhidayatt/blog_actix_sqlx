use std::sync::Arc;

use crate::{
    domain::{PostRelationResponse, PostResponse},
    models::{PostModel, PostRelationModel},
};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub type DynPostsRepository = Arc<dyn PostsRepositoryTrait + Send + Sync>;
pub type DynPostsService = Arc<dyn PostsServiceTrait + Send + Sync>;

#[async_trait]
pub trait PostsRepositoryTrait {
    async fn get_all_posts(&self) -> Result<Vec<PostModel>>;
    async fn get_post(&self, post_id: Uuid) -> Result<Option<PostModel>>;
    async fn get_post_relation(&self, post_id: Uuid) -> Result<Vec<PostRelationModel>>;
    async fn create_post(
        &self,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<PostModel>;
    async fn update_post(
        &self,
        post_id: Uuid,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<Option<PostModel>>;
    async fn delete_post(&self, post_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait PostsServiceTrait {
    async fn get_all_posts(&self) -> Result<Vec<PostResponse>>;
    async fn get_post(&self, post_id: Uuid) -> Result<Option<PostResponse>>;
    async fn get_post_relation(&self, post_id: Uuid) -> Result<Vec<PostRelationResponse>>;
    async fn create_post(
        &self,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<PostResponse>;
    async fn update_post(
        &self,
        post_id: Uuid,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<Option<PostResponse>>;
    async fn delete_post(&self, post_id: Uuid) -> Result<()>;
}

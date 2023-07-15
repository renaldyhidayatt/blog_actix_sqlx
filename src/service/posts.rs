use crate::abstract_trait::{DynPostsRepository, PostsServiceTrait};
use crate::domain::{PostRelationResponse, PostResponse};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct PostsService {
    repository: DynPostsRepository,
}

impl PostsService {
    pub fn new(repository: DynPostsRepository) -> Self {
        PostsService { repository }
    }
}

#[async_trait]
impl PostsServiceTrait for PostsService {
    async fn get_all_posts(&self) -> Result<Vec<PostResponse>> {
        let posts = self.repository.get_all_posts().await?;
        let post_responses: Vec<PostResponse> = posts.into_iter().map(|post| post.into()).collect();
        Ok(post_responses)
    }

    async fn get_post(&self, post_id: Uuid) -> Result<Option<PostResponse>> {
        let post = self.repository.get_post(post_id).await?;
        match post {
            Some(post) => {
                let post_response: PostResponse = post.into();
                Ok(Some(post_response))
            }
            None => Ok(None),
        }
    }

    async fn get_post_relation(&self, post_id: Uuid) -> Result<Vec<PostRelationResponse>> {
        let post_relations = self.repository.get_post_relation(post_id).await?;
        let post_relation_responses: Vec<PostRelationResponse> = post_relations
            .into_iter()
            .map(|post_relation| post_relation.into())
            .collect();
        Ok(post_relation_responses)
    }

    async fn create_post(
        &self,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<PostResponse> {
        let post = self
            .repository
            .create_post(title, body, category_id, user_id, user_name)
            .await?;
        let post_response: PostResponse = post.into();
        Ok(post_response)
    }

    async fn update_post(
        &self,
        post_id: Uuid,
        title: &str,
        body: &str,
        category_id: i32,
        user_id: i32,
        user_name: &str,
    ) -> Result<Option<PostResponse>> {
        let post = self
            .repository
            .update_post(post_id, title, body, category_id, user_id, user_name)
            .await?;
        match post {
            Some(post) => {
                let post_response: PostResponse = post.into();
                Ok(Some(post_response))
            }
            None => Ok(None),
        }
    }

    async fn delete_post(&self, post_id: Uuid) -> Result<()> {
        self.repository.delete_post(post_id).await?;
        Ok(())
    }
}

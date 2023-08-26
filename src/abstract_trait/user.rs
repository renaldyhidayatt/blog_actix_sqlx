use anyhow::Result;
use std::sync::Arc;

use async_trait::async_trait;

use crate::{domain::UserResponse, models::UserModel};

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_by_email_exists(&self, email: &str) -> Result<bool>;
    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserModel>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<UserModel>>;
    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> Result<Option<UserModel>>;
    async fn delete_user(&self, email: &str) -> Result<bool>;
}

#[async_trait]
pub trait UserServiceTrait {
    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<UserResponse>;
    async fn find_by_email_exists(&self, email: &str) -> anyhow::Result<bool>;
    async fn find_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserModel>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<UserResponse>>;
    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> anyhow::Result<Option<UserResponse>>;
    async fn delete_user(&self, email: &str) -> anyhow::Result<bool>;
}

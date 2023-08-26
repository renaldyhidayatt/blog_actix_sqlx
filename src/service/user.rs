use async_trait::async_trait;

use crate::abstract_trait::{DynUserRepository, UserServiceTrait};
use crate::domain::UserResponse;
use crate::models::UserModel;

#[derive(Clone)]
pub struct UserService {
    pub repository: DynUserRepository,
}

impl UserService {
    pub fn new(repository: DynUserRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<UserResponse> {
        let user = self
            .repository
            .create_user(firstname, lastname, email, password)
            .await?;
        Ok(user.into())
    }

    async fn find_by_email_exists(&self, email: &str) -> anyhow::Result<bool> {
        self.repository
            .find_by_email_exists(email)
            .await
            .map_err(|err| err.into())
    }

    async fn find_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserModel>> {
        self.repository
            .find_by_email(email)
            .await
            .map_err(|err| err.into())
    }

    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<UserResponse>> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user.map(|u| u.into()))
    }

    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> anyhow::Result<Option<UserResponse>> {
        let user = self
            .repository
            .update_user(email, firstname, lastname, password)
            .await?;
        Ok(user.map(|u| u.into()))
    }

    async fn delete_user(&self, email: &str) -> anyhow::Result<bool> {
        Ok(self
            .repository
            .delete_user(email)
            .await
            .expect("Error Delete"))
    }
}

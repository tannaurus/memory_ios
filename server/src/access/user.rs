use async_trait::async_trait;
use uuid::Uuid;

use crate::{auth::VerifiedUser, model};

use super::{schema, AccessError, MemoryDb};

#[async_trait]
pub trait AccessUser {
    async fn create_user(&self, name: String) -> Result<model::User, AccessError>;
    async fn get_user(&self, user: &VerifiedUser) -> Result<model::User, AccessError>;
}

#[async_trait]
impl AccessUser for MemoryDb {
    async fn create_user(&self, name: String) -> Result<model::User, AccessError> {
        let uuid = Uuid::new_v4();
        let user_id = sqlx::query!(
            "INSERT INTO users (name, uuid) VALUES (?, ?)",
            name,
            uuid.to_string()
        )
        .execute(&self.inner)
        .await?
        .last_insert_id();

        let user = sqlx::query_as!(schema::User, "SELECT * FROM users WHERE id = ?", user_id)
            .fetch_one(&self.inner)
            .await?
            .try_into()?;

        Ok(user)
    }

    async fn get_user(&self, user: &VerifiedUser) -> Result<model::User, AccessError> {
        let user = sqlx::query_as!(schema::User, "SELECT * FROM users WHERE id = ?", user.id()?)
            .fetch_one(&self.inner)
            .await?
            .try_into()?;

        Ok(user)
    }
}

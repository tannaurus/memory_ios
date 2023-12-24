use async_trait::async_trait;

use crate::{access::schema, model};

use super::{AccessError, MemoryDb};

#[async_trait]
pub trait AccessPrompt {
    async fn get_prompts(&self) -> Result<Vec<model::Prompt>, AccessError>;
}

#[async_trait]
impl AccessPrompt for MemoryDb {
    async fn get_prompts(&self) -> Result<Vec<model::Prompt>, AccessError> {
        let rows = sqlx::query_as!(schema::Prompt, "SELECT * FROM prompts")
            .fetch_all(&self.inner)
            .await?;

        let mut prompts = Vec::new();
        for prompt in rows {
            prompts.push(prompt.try_into()?);
        }

        Ok(prompts)
    }
}

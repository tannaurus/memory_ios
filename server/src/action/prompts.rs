use crate::{access::prompts::AccessPrompt, model};

use super::ActionError;

pub async fn get_prompts<A>(db: A) -> Result<Vec<model::Prompt>, ActionError>
where
    A: AccessPrompt,
{
    Ok(db.get_prompts().await?)
}

use anyhow::Context;
use serde::de::DeserializeOwned;
use std::{fmt::Display, fs::File, io::BufReader};

use crate::AppError;

#[derive(Debug)]
pub enum DbEntity {
    Prompts,
    Stories,
    Users,
}

impl Display for DbEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

pub(crate) fn read_db<T>(entity: DbEntity, uuid: &str) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let path = format!("db/{}/{}", entity, uuid);
    let file = File::open(&path).context(format!("Failed to open {}", path))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).context(format!("Failed to deserialize {}", path))?)
}

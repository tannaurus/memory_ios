use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
};

use crate::AppError;

#[derive(Debug)]
pub enum DbEntity {
    Prompts,
    Stories,
    Content,
    Users,
}

impl Display for DbEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// Returns deserialized T, if found.
/// Returns [AppError] if file does not exist.
pub fn read_db<T>(kind: DbEntity, uuid: &str) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let path = format!("db/{}/{}.json", kind, uuid);
    let file = File::open(&path).context(format!("Failed to open {}", path))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).context(format!("Failed to deserialize {}", path))?)
}

/// Writes T into a new json file.
/// Returns [AppError] if something goes wrong.
pub fn write_db<T>(kind: DbEntity, uuid: &str, data: &T) -> Result<(), AppError>
where
    T: ?Sized + Serialize,
{
    let path = format!("db/{}/{}.json", kind, uuid);
    let file = File::create(&path).context(format!("Failed to create file {}", path))?;
    let writer = BufWriter::new(file);
    Ok(serde_json::to_writer(writer, data).context(format!("Failed to write {}", path))?)
}

use anyhow::Context;
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader};

use crate::AppError;

pub(crate) fn read_mocked_data<T>(filename: &str) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let file = File::open(format!("src/mock_api/{}", filename))
        .context(format!("Failed to open {}", filename))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).context(format!("Failed to deserialize {}", filename))?)
}

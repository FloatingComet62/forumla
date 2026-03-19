use crate::{Errors, Result};
use std::{fs, path::PathBuf};
use tracing::{instrument, warn};

#[derive(Debug)]
pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| {
                warn!("Could not determine cache directory");
                Errors::FailedToSetupCache
            })?.join("forumla");
        fs::create_dir_all(&cache_dir).map_err(|e| {
            warn!("Failed to create cache directory: {:?}", e);
            Errors::FailedToSetupCache
        })?;

        Ok(Self {
            cache_dir
        })
    }

    #[instrument]
    pub fn set(&self, key: &str, value: &serde_json::Value) -> Result<()> {
        fs::write(
            self.cache_dir.join(format!("{}.json", key)), 
            value.to_string()
        ).map_err(|e| {
            warn!("Cache file error: {:?}", e);
            Errors::FailedToSaveCache
        })?;
        Ok(())
    }

    #[instrument]
    pub fn get(&self, key: &str) -> Result<serde_json::Value> {
        let file_name = format!("{}.json", key);
        let data = fs::read(self.cache_dir.join(file_name.clone())).map_err(|_| {
            warn!("Failed to read file {:?}", file_name);
            Errors::FailedToLoadCache
        })?;
        let s = str::from_utf8(&data).map_err(|_| {
            warn!("Cache content is invalid UTF8");
            Errors::FailedToLoadCache
        })?;
        let output = serde_json::from_str(s).map_err(|_| {
            warn!("Cache content is invalid JSON");
            Errors::FailedToLoadCache
        })?;

        Ok(output)
    }

    pub fn purge(&self) -> Result<()> {
        fs::remove_dir_all(&self.cache_dir).map_err(|e| {
            warn!("Failed to purge cache: {:?}", e);
            Errors::FailedToSetupCache
        })?;

        fs::create_dir_all(&self.cache_dir).map_err(|e| {
            warn!("Failed to recreate cache directory after purge: {:?}", e);
            Errors::FailedToSetupCache
        })?;

        Ok(())
    }
}

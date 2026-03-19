use crate::{cache::Cache, Errors, Result};
use tracing::{debug, instrument, warn};

#[derive(Debug)]
pub struct RequestClient {
    base_url: String,
    client: reqwest::Client,
    cache: Option<Cache>,
}

impl RequestClient {
    pub fn new(cache: Option<Cache>) -> Self {
        Self {
            base_url: "https://api.jolpi.ca/".to_string(),
            client: reqwest::Client::new(),
            cache,
        }
    }

    async fn request_nocached(&self, path: &str) -> Result<serde_json::Value> {
        let response = self.client.get(self.base_url.clone() + path).send().await.map_err(|e| {
            warn!("Request failed: {:?}", e);
            Errors::RequestFailed
        })?;
        let data = response.json::<serde_json::Value>().await.map_err(|e| {
            warn!("Invalid response body: {:?}", e);
            Errors::RequestFailed
        })?;
        Ok(data)
    }

    async fn request(&self, path: &str) -> Result<serde_json::Value> {
        let Some(cache) = &self.cache else {
            return self.request_nocached(path).await;
        };
        let key = path.replace("/", "_");
        if let Ok(cached_data) = cache.get(&key) {
            return Ok(cached_data);
        }

        let data = self.request_nocached(path).await?;
        if let Err(e) = cache.set(&key, &data) {
            warn!("Failed to save cache: {:?}", e);
        }
        Ok(data)
    }

    #[instrument]
    pub async fn health_check(&self) -> Result<()> {
        let data = self.request_nocached("").await?;
        debug!("Health Check: {:?}", data);
        Ok(())
    }

    #[instrument]
    pub async fn list_all_f1_circuits(&self) -> Result<()> {
        let data = self.request("f1/alpha/core/circuits").await?;
        debug!("{:?}", data);
        Ok(())
    }

    #[instrument]
    pub async fn list_all_f1_circuit(&self, api_id: &str) -> Result<()> {
        let data = self.request(&format!("f1/alpha/core/circuits/{:?}", api_id)).await?;
        debug!("{:?}", data);
        Ok(())
    }

    #[instrument]
    pub async fn list_all_f1_drivers(&self) -> Result<()> {
        let data = self.request("f1/alpha/core/drivers").await?;
        debug!("{:?}", data);
        Ok(())
    }

    #[instrument]
    pub async fn list_all_f1_driver(&self, api_id: &str) -> Result<()> {
        let data = self.request(&format!("f1/alpha/core/drivers/{:?}", api_id)).await?;
        debug!("{:?}", data);
        Ok(())
    }

    #[instrument]
    pub async fn list_all_f1_laps(&self) -> Result<()> {
        let data = self.request("f1/alpha/core/laps").await?;
        debug!("{:?}", data);
        Ok(())
    }

    #[instrument]
    pub async fn list_all_f1_lap(&self, api_id: &str) -> Result<()> {
        let data = self.request(&format!("f1/alpha/core/laps/{:?}", api_id)).await?;
        debug!("{:?}", data);
        Ok(())
    }
}

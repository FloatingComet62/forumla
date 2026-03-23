use crate::{Errors, Result, cache::Cache, params::*, schemas::*};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub metadata: Metadata,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseSingle<T> {
    pub metadata: StrippedMetadata,
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StrippedMetadata {
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub timestamp: String,
    pub count: u32,
    pub page_size: u32,
    pub current_page: u32,
    pub total_pages: u32,
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
}

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
        let response = self
            .client
            .get(self.base_url.clone() + path)
            .send()
            .await
            .map_err(|e| {
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
        let mut key = path.replace("/", "_");
        key = key.replace("?", "_");
        key = key.replace("=", "_");
        key = key.replace("%", "_");
        key = key.replace("-", "_");
        key = key.replace(".", "_");
        key = key.replace("~", "_");
        if let Ok(cached_data) = cache.get(&key) {
            return Ok(cached_data);
        }

        let data = self.request_nocached(path).await?;
        if let Err(e) = cache.set(&key, &data) {
            warn!("Failed to save cache: {:?}", e);
        }
        Ok(data)
    }

    pub async fn health_check(&self) -> Result<()> {
        let data = self.request_nocached("").await?;
        debug!("Health Check: {:?}", data);
        Ok(())
    }

    pub async fn list_all_circuits(&self, params: CircuitParams) -> Result<ApiResponse<Circuit>> {
        let json = self
            .request(&("f1/alpha/core/circuits".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_circuit(&self, api_id: &str) -> Result<ApiResponseSingle<Circuit>> {
        let json = self
            .request(&format!("f1/alpha/core/circuits/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_drivers(&self, params: DriverParams) -> Result<ApiResponse<Driver>> {
        let json = self
            .request(&("f1/alpha/core/drivers".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_driver(&self, api_id: &str) -> Result<ApiResponseSingle<Driver>> {
        let json = self
            .request(&format!("f1/alpha/core/drivers/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_laps(&self, params: LapParams) -> Result<ApiResponse<Lap>> {
        let json = self
            .request(&("f1/alpha/core/laps".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_lap(&self, api_id: &str) -> Result<ApiResponseSingle<Lap>> {
        let json = self
            .request(&format!("f1/alpha/core/laps/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_pitstops(&self, params: PitStopParams) -> Result<ApiResponse<PitStop>> {
        let json = self
            .request(&("f1/alpha/core/pit-stops".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_pitstop(&self, api_id: &str) -> Result<ApiResponseSingle<PitStop>> {
        let json = self
            .request(&format!("f1/alpha/core/pit-stops/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_rounds(&self, params: RoundParams) -> Result<ApiResponse<Round>> {
        let json = self
            .request(&("f1/alpha/core/rounds".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_round(&self, api_id: &str) -> Result<ApiResponseSingle<Round>> {
        let json = self
            .request(&format!("f1/alpha/core/rounds/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_seasons(&self, params: SeasonParams) -> Result<ApiResponse<Season>> {
        let json = self
            .request(&("f1/alpha/core/seasons".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_season(&self, api_id: &str) -> Result<ApiResponseSingle<Season>> {
        let json = self
            .request(&format!("f1/alpha/core/seasons/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_session_entries(
        &self,
        params: SessionEntryParams,
    ) -> Result<ApiResponse<SessionEntry>> {
        let json = self
            .request(&("f1/alpha/core/session-entries".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_session_entry(&self, api_id: &str) -> Result<ApiResponseSingle<SessionEntry>> {
        let json = self
            .request(&format!("f1/alpha/core/session-entries/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_sessions(&self, params: SessionParams) -> Result<ApiResponse<Session>> {
        let json = self
            .request(&("f1/alpha/core/sessions".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_session(&self, api_id: &str) -> Result<ApiResponseSingle<Session>> {
        let json = self
            .request(&format!("f1/alpha/core/sessions/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn list_all_teams(&self, params: TeamParams) -> Result<ApiResponse<Team>> {
        let json = self
            .request(&("f1/alpha/core/teams".to_string() + &params.search_params()))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    pub async fn get_team(&self, api_id: &str) -> Result<ApiResponseSingle<Team>> {
        let json = self
            .request(&format!("f1/alpha/core/teams/{:?}", api_id))
            .await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }
}

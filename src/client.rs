use crate::{cache::Cache, Errors, Result};
use tracing::{debug, instrument, warn};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Circuit {
    pub id: String,
    pub url: String,
    pub name: String,
    pub locality: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub wikipedia: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Driver {
    pub id: String,
    pub url: String,
    pub abbreviation: Option<String>,
    pub given_name: String,
    pub family_name: String,
    pub nationality: Option<String>,
    pub country_code: Option<String>,
    pub permanent_car_number: Option<String>,
    pub date_of_birth: Option<String>,
    pub wikipedia: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Lap {
    pub id: String,
    pub url: String,
    pub number: Option<String>,
    pub position: Option<String>,
    pub time: Option<String>,
    pub time_display: Option<String>,
    pub time_milliseconds: Option<u64>,

    // not sure if it's actually string, i can't verify because
    // i can't find a doc which actually includes this average speed
    pub average_speed: Option<String>,
    pub is_entry_fastest_lap: bool,
    pub session_entry: LapSessionEntry,
    pub pit_stop: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LapSessionEntry {
    pub id: String,
    pub url: String
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStop {
    pub id: String,
    pub url: String,
    pub number: Option<String>,
    pub duration: Option<String>,
    pub duration_display: Option<String>,
    pub duration_milliseconds: Option<u64>,
    pub local_timestamp: Option<String>,
    pub driver: PitStopDriver,
    pub team: PitStopTeam,
    pub session: PitStopSession,
    pub round: PitStopRound,
    pub season: PitStopSeason,
    pub lap: Option<PitStopLap>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStopDriver {
    pub id: String,
    pub url: String,
    pub abbreviation: Option<String>,
    pub given_name: String,
    pub family_name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStopTeam {
    pub id: String,
    pub url: String,
    pub name: String,
    pub primary_color: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStopSession {
    pub id: String,
    pub url: String,
    pub number: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub type_display: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStopRound {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    pub name: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStopSeason {
    pub id: String,
    pub url: String,
    pub year: u64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PitStopLap {
    pub id: String,
    pub url: String,
    pub number: u64,
    pub position: u64,
    pub time: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Round {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    pub name: Option<String>,
    pub race_number: Option<u64>,
    pub wikipedia: Option<String>,
    pub is_cancelled: bool,
    pub circuit: RoundCircuit,
    pub season: RoundSeason,
    pub sessions: Vec<RoundSessions>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RoundCircuit {
    pub id: String,
    pub url: String,
    pub name: String,
    pub locality: Option<String>,
    pub country_code: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RoundSeason {
    pub id: String,
    pub url: String,
    pub year: u64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RoundSessions {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    #[serde(rename = "type")]
    pub kind: String,
    pub type_display: String,
    pub timestamp: Option<String>,
    pub missing_time_data: Option<String>,
    pub local_timestamp: Option<String>,
    pub timezone: Option<String>,
    pub scheduled_laps: Option<u64>,
    pub is_cancelled: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub id: String,
    pub url: String,
    pub year: u64,
    pub wikipedia: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEntry {
    pub id: String,
    pub url: String,
    pub position: Option<u64>,
    pub is_classified: Option<bool>,
    pub status: Option<u64>,
    pub status_display: Option<String>,
    pub points: Option<u64>,
    pub grid: Option<u64>,
    pub time: Option<String>,
    pub time_display: Option<String>,
    pub fastest_lap_rank: Option<u64>,
    pub laps_completed: Option<u64>,
    pub session: SessionEntrySession,
    pub round: SessionEntryRound,
    pub driver: SessionEntryDriver,
    pub team: SessionEntryTeam,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEntrySession {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    #[serde(rename = "type")]
    pub kind: String,
    pub type_display: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEntryRound {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    pub name: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEntryDriver {
    pub id: String,
    pub url: String,
    pub abbreviation: Option<String>,
    pub given_name: String,
    pub family_name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEntryTeam {
    pub id: String,
    pub url: String,
    pub name: String,
    pub primary_color: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    #[serde(rename = "type")]
    pub kind: String,
    pub type_display: String,
    pub timestamp: Option<String>,
    pub missing_time_data: Option<String>,
    pub local_timestamp: Option<String>,
    pub timezone: Option<String>,
    pub scheduled_laps: Option<String>,
    pub is_cancelled: bool,
    pub round: SessionRound,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionRound {
    pub id: String,
    pub url: String,
    pub number: Option<u64>,
    pub name: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    pub id: String,
    pub url: String,
    pub name: String,
    pub primary_color: Option<String>,
    pub nationality: Option<String>,
    pub country_code: Option<String>,
    pub wikipedia: Option<String>,
    pub seasons: Vec<TeamSeason>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TeamSeason {
    pub id: String,
    pub url: String,
    pub year: u64,
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
    pub async fn list_all_circuits(&self) -> Result<ApiResponse<Circuit>> {
        let json = self.request("f1/alpha/core/circuits").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_circuit(&self, api_id: &str) -> Result<ApiResponseSingle<Circuit>> {
        let json = self.request(&format!("f1/alpha/core/circuits/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_drivers(&self) -> Result<ApiResponse<Driver>> {
        let json = self.request("f1/alpha/core/drivers").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_driver(&self, api_id: &str) -> Result<ApiResponseSingle<Driver>> {
        let json = self.request(&format!("f1/alpha/core/drivers/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_laps(&self) -> Result<ApiResponse<Lap>> {
        let json = self.request("f1/alpha/core/laps").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_lap(&self, api_id: &str) -> Result<ApiResponseSingle<Lap>> {
        let json = self.request(&format!("f1/alpha/core/laps/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_pitstops(&self) -> Result<ApiResponse<PitStop>> {
        let json = self.request("f1/alpha/core/pit-stops").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_pitstop(&self, api_id: &str) -> Result<ApiResponseSingle<PitStop>> {
        let json = self.request(&format!("f1/alpha/core/pit-stops/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_rounds(&self) -> Result<ApiResponse<Round>> {
        let json = self.request("f1/alpha/core/rounds").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_round(&self, api_id: &str) -> Result<ApiResponseSingle<Round>> {
        let json = self.request(&format!("f1/alpha/core/rounds/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_seasons(&self) -> Result<ApiResponse<Season>> {
        let json = self.request("f1/alpha/core/seasons").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_season(&self, api_id: &str) -> Result<ApiResponseSingle<Season>> {
        let json = self.request(&format!("f1/alpha/core/seasons/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_session_entries(&self) -> Result<ApiResponse<SessionEntry>> {
        let json = self.request("f1/alpha/core/session-entries").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_session_entry(&self, api_id: &str) -> Result<ApiResponseSingle<SessionEntry>> {
        let json = self.request(&format!("f1/alpha/core/session-entries/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_sessions(&self) -> Result<ApiResponse<Session>> {
        let json = self.request("f1/alpha/core/sessions").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_session(&self, api_id: &str) -> Result<ApiResponseSingle<Session>> {
        let json = self.request(&format!("f1/alpha/core/sessions/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn list_all_teams(&self) -> Result<ApiResponse<Team>> {
        let json = self.request("f1/alpha/core/teams").await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }

    #[instrument]
    pub async fn get_team(&self, api_id: &str) -> Result<ApiResponseSingle<Team>> {
        let json = self.request(&format!("f1/alpha/core/teams/{:?}", api_id)).await?;
        serde_json::from_value(json).map_err(|e| {
            warn!("Invalid response schema received: {:?}", e);
            Errors::InvalidResponseSchema
        })
    }
}

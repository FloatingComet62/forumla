use serde::{Deserialize, Serialize};

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
    pub permanent_car_number: Option<u32>,
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
    pub url: String,
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

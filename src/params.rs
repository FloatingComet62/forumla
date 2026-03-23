use std::fmt::Display;

use params_macro::{Params, params};

#[derive(Clone)]
pub enum DriverRole {
    Permanent,
    Reserve,
    Junior,
}
impl Display for DriverRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Self::Permanent => 0,
                Self::Reserve => 1,
                Self::Junior => 2,
            }
        ))
    }
}

#[derive(Clone)]
pub enum SessionType {
    Race,
    QualifyingOne,
    QualifyingTwo,
    QualifyingThree,
    QUalifyingAggregate,
    QualifyingOrder,
    QualifyingBest,
    PracticeOne,
    PracticeTwo,
    PracticeThree,
    PreQualifying,
    SprintRace,
    SprintQualifyingOne,
    SprintQualifyingTwo,
    SprintQualifyingThree,
}
impl Display for SessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Self::Race => "R",
                Self::QualifyingOne => "Q1",
                Self::QualifyingTwo => "Q2",
                Self::QualifyingThree => "Q3",
                Self::QUalifyingAggregate => "QA",
                Self::QualifyingOrder => "QO",
                Self::QualifyingBest => "QB",
                Self::PracticeOne => "FP1",
                Self::PracticeTwo => "FP2",
                Self::PracticeThree => "FP3",
                Self::PreQualifying => "PQ",
                Self::SprintRace => "SR",
                Self::SprintQualifyingOne => "SQ1",
                Self::SprintQualifyingTwo => "SQ2",
                Self::SprintQualifyingThree => "SQ3",
            }
        ))
    }
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct CircuitParams {
    pub year: u32,
    pub page: u32,

    pub country_code: String,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct DriverParams {
    pub year: u32,
    pub page: u32,

    pub country_code: String,
    pub team_id: String,
    pub role: DriverRole,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct LapParams {
    pub year: u32,
    pub page: u32,

    pub driver_id: String,
    pub session_entry_id: String,
    pub session_id: String,
    pub team_id: String,
    pub has_pit_stop: bool,
    pub is_fastest_lap: bool,
    pub session_type: SessionType,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct PitStopParams {
    pub year: u32,
    pub page: u32,

    pub driver_id: String,
    pub round_id: String,
    pub session_id: String,
    pub team_id: String,
    pub lap_number: u32,
    pub stop_number: u32,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct RoundParams {
    pub year: u32,
    pub page: u32,

    pub driver_id: String,
    pub team_id: String,
    pub race_number: u32,
    pub round_number: u32,
    pub is_cancelled: bool,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct SeasonParams {
    pub page: u32,

    pub country_code: String,
    pub driver_id: String,
    pub team_id: String,
    pub circuit_id: String,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct SessionEntryParams {
    pub year: u32,
    pub page: u32,

    pub has_session_points: bool,
    pub driver_id: String,
    pub round_id: String,
    pub session_id: String,
    pub team_id: String,
    pub session_type: SessionType,
    pub position: u32,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct SessionParams {
    pub year: u32,
    pub page: u32,

    pub round_id: String,
    pub circuit_id: String,
    pub session_type: SessionType,
}

#[derive(Params, Default)]
#[params(all_optional)]
pub struct TeamParams {
    pub year: u32,
    pub page: u32,

    pub country_code: String,
}

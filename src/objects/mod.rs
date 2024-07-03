use serde::Deserialize;
use crate::api;

#[derive(Deserialize)]
pub struct Races {
    pub season: String,
    pub round: String,
    #[serde(rename(deserialize = "raceName"))]
    pub race_name: String,
    #[serde(rename(deserialize = "Circuit"))]
    pub circuit: Circuits,
    pub date: String,
    pub time: String,
    #[serde(rename(deserialize = "FirstPractice"))]
    pub fp1: Option<RaceInfo>,
    #[serde(rename(deserialize = "SecondPractice"))]
    pub fp2: Option<RaceInfo>,
    #[serde(rename(deserialize = "ThirdPractice"))]
    pub fp3: Option<RaceInfo>,
    #[serde(rename(deserialize = "Qualifying"))]
    pub quali: Option<RaceInfo>,
    #[serde(rename(deserialize = "Sprint"))]
    pub sprint: Option<RaceInfo>,
}

#[derive(Deserialize)]
pub struct Circuits {
    #[serde(rename(deserialize = "circuitName"))]
    pub circuit_name: String,
    #[serde(rename(deserialize = "Location"))]
    pub location: Local,
}

#[derive(Deserialize)]
pub struct RaceInfo {
    pub date: String,
    pub time: String,
}

#[derive(Deserialize)]
pub struct Local {
    pub locality: String,
    pub country: String,
}

impl Races {
    // Will deserialize from json
    pub fn from_json(url: &str) -> Vec<Self> {
        let races = serde_json::from_value(api::api_pull(url));
        races.expect("Problem converting data")
    }

    // Will deserialize from json
    pub fn from_ergast_json() -> Vec<Self> {
        serde_json::from_value(api::api_pull("http://ergast.com/api/f1/current.json"))
            .expect("Problem converting data")
    }
}

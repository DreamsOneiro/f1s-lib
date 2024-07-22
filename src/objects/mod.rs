use crate::{api, libraries::time};
use chrono::{Utc, DateTime};
use serde::{de::DeserializeOwned, Deserialize};

fn deserialize_obj<T: DeserializeOwned>(json_str: &str) -> T {
    serde_json::from_str(json_str).expect("Problem converting data")
}

#[derive(Deserialize)]
pub struct Races {
    pub year: usize,
    pub round: usize,
    #[serde(rename(deserialize = "date"))]
    mr_date: String,
    #[serde(rename(deserialize = "time"))]
    mr_time: String,
    #[serde(rename(deserialize = "qualifying_date"))]
    quali_date: String,
    #[serde(rename(deserialize = "qualifying_time"))]
    quali_time: String,
    #[serde(rename(deserialize = "free_practice_1_date"))]
    fp1_date: String,
    #[serde(rename(deserialize = "free_practice_1_time"))]
    fp1_time: String,
    #[serde(rename(deserialize = "free_practice_2_date"))]
    fp2_date: Option<String>,
    #[serde(rename(deserialize = "free_practice_2_time"))]
    fp2_time: Option<String>,
    #[serde(rename(deserialize = "free_practice_3_date"))]
    fp3_date: Option<String>,
    #[serde(rename(deserialize = "free_practice_3_time"))]
    fp3_time: Option<String>,
    #[serde(rename(deserialize = "sprint_qualifying_date"))]
    sq_date: Option<String>,
    #[serde(rename(deserialize = "sprint_qualifying_time"))]
    sq_time: Option<String>,
    #[serde(rename(deserialize = "sprint_race_date"))]
    sprint_date: Option<String>,
    #[serde(rename(deserialize = "sprint_race_time"))]
    sprint_time: Option<String>,
    pub circuit: String,
    pub country: String,
    pub locality: String,
    pub grand_prix: String,
}
impl Races {
    pub fn new() -> Vec<Races> {
        let response = api::API::new_github().request();
        deserialize_obj(&response.json)
    }

    pub fn main_race(&self) -> DateTime<Utc> {
        time::to_dt(&self.mr_date, &self.mr_time)
    }

    pub fn quali(&self) -> DateTime<Utc> {
        time::to_dt(&self.quali_date, &self.quali_time)
    }

    pub fn fp1(&self) -> DateTime<Utc> {
        time::to_dt(&self.fp1_date, &self.fp1_time)
    }

    pub fn fp2(&self) -> DateTime<Utc> {
        time::to_dt(&self.fp2_date.as_ref().unwrap(), &self.fp2_time.as_ref().unwrap())
    }

    pub fn fp3(&self) -> DateTime<Utc> {
            time::to_dt(&self.fp3_date.as_ref().unwrap(), &self.fp3_time.as_ref().unwrap())
    }

    pub fn sq(&self) -> DateTime<Utc> {
            time::to_dt(&self.sq_date.as_ref().unwrap(), &self.sq_time.as_ref().unwrap())
    }

    pub fn sprint(&self) -> DateTime<Utc> {
            time::to_dt(&self.sprint_date.as_ref().unwrap(), &self.sprint_time.as_ref().unwrap())
    }

    pub fn has_sprint(&self) -> bool {
        self.sprint_date != None
    }
}

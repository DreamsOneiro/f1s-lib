use crate::{api, libraries::time};
use chrono::{Utc, DateTime};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Race {
    pub year: usize,
    pub round: usize,
    grand_prix_id: String,
    circuit_id: String,
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
    circuit: Option<String>,
    country: Option<String>,
    locality: Option<String>,
    grand_prix: Option<String>,
}
impl Race {
    pub fn new() -> Vec<Race> {
        let response = Self::new_request();
        let race = response.0;
        let cc = Circuit::to_hash(response.1);
        let ct = Country::to_hash(response.2);
        let gp = GrandPrix::to_hash(response.3);
        race.into_iter().map(|r| r.init(&cc, &ct, &gp)).collect()
    }

    pub fn circuit(&self) -> &str {
        self.circuit.as_ref().unwrap()
    }

    pub fn country(&self) -> &str {
        self.country.as_ref().unwrap()
    }

    pub fn locality(&self) -> &str {
        self.locality.as_ref().unwrap()
    }

    pub fn grand_prix(&self) -> &str {
        self.grand_prix.as_ref().unwrap()
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

    fn new_request() -> (Vec<Race>, Vec<Circuit>, Vec<Country>, Vec<GrandPrix>) {
        let response = api::API::new_github().request();
        let race: Vec<Race> = deserialize_obj(&response.race_json);
        let circuit: Vec<Circuit> = deserialize_obj(&response.circuit_json);
        let country: Vec<Country> = deserialize_obj(&response.country_json);
        let grand_prix: Vec<GrandPrix> = deserialize_obj(&response.grand_prix_json);
        return (race, circuit, country, grand_prix);
    }

    fn init(
        mut self,
        cc: &HashMap<String, Circuit>,
        ct: &HashMap<String, Country>,
        gp: &HashMap<String, GrandPrix>,
    ) -> Self {
        let circuit = cc.get(&self.circuit_id).unwrap();
        let grand_prix = gp.get(&self.grand_prix_id).unwrap();
        let country = ct.get(&circuit.country_id).unwrap();
        self.circuit = Some(circuit.name.to_string());
        self.country = Some(country.name.to_string());
        self.locality = Some(circuit.locality.to_string());
        self.grand_prix = Some(grand_prix.name.to_string());
        self
    }
}

#[derive(Deserialize)]
struct Circuit {
    id: String,
    #[serde(rename(deserialize = "full_name"))]
    name: String,
    #[serde(rename(deserialize = "place_name"))]
    locality: String,
    country_id: String,
}
impl Circuit {
    fn to_hash(obj: Vec<Circuit>) -> HashMap<String, Circuit> {
        let mut map = HashMap::new();
        for i in obj {
            map.insert(i.id.to_string(), i);
        }
        return map;
    }
}

#[derive(Deserialize)]
struct GrandPrix {
    id: String,
    #[serde(rename(deserialize = "full_name"))]
    name: String,
}
impl GrandPrix {
    fn to_hash(obj: Vec<GrandPrix>) -> HashMap<String, GrandPrix> {
        let mut map = HashMap::new();
        for i in obj {
            map.insert(i.id.to_string(), i);
        }
        return map;
    }
}

#[derive(Deserialize)]
struct Country {
    id: String,
    name: String,
}
impl Country {
    fn to_hash(obj: Vec<Country>) -> HashMap<String, Country> {
        let mut map = HashMap::new();
        for i in obj {
            map.insert(i.id.to_string(), i);
        }
        return map;
    }
}

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
        deserialize_obj(&response.full_json)
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

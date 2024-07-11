pub struct API {
    pub race_url: String,
    pub circuit_url: String,
    pub grand_prix_url: String,
    pub country_url: String,
    pub race_json: String,
    pub circuit_json: String,
    pub grand_prix_json: String,
    pub country_json: String
} 
impl API {
    pub fn new_github() -> Self {
        let url_base = String::from("https://raw.githubusercontent.com/DreamsOneiro/f1s-api/main");
        API {
            race_url: format!("{url_base}/race.json"),
            circuit_url: format!("{url_base}/circuit.json"),
            grand_prix_url: format!("{url_base}/grand_prix.json"),
            country_url: format!("{url_base}/country.json"),
            race_json: String::new(),
            circuit_json: String::new(),
            grand_prix_json: String::new(),
            country_json: String::new()
        }
    }

    pub fn request(mut self) -> Self {
        self.race_json = reqwest_url(&self.race_url);
        self.circuit_json = reqwest_url(&self.circuit_url);
        self.grand_prix_json = reqwest_url(&self.grand_prix_url);
        self.country_json = reqwest_url(&self.country_url);

        fn reqwest_url(url: &str) -> String {
            reqwest::blocking::get(url)
                .expect(&format!("Problem retreiving data from: {url}, check connection"))
                .text()
                .expect(&format!("Problem converting data from: {url} to text"))
        }
        self
    }
}

// Depreciated, Ergast will be shutting down end of 2024
pub fn api_pull(url: &str) -> serde_json::Value {
    let data = reqwest::blocking::get(url)
        .expect("Problem retreiving data, check connection")
        .text()
        .expect("Problem converting to text");
    let data: serde_json::Value = serde_json::from_str(&data)
        .expect("Problem converting data");
    data.get("MRData")
        .expect("Code 1: Problem reading data")
        .get("RaceTable")
        .expect("Code 2: Problem reading data")
        .get("Races")
        .expect("Code 3: Problem reading data")
        .clone()
}

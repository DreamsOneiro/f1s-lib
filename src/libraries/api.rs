pub struct API {
    pub url: String,
    pub json: String,
} 
impl API {
    pub fn new_github() -> Self {
        let url_base = String::from("https://raw.githubusercontent.com/DreamsOneiro/f1s-api/main");
        API {
            url: format!("{url_base}/f1db.json"),
            json: String::new(),
        }
    }

    pub fn request(mut self) -> Self {
        self.json = reqwest_url(&self.url);
        return self;

        fn reqwest_url(url: &str) -> String {
            reqwest::blocking::get(url)
                .expect(&format!("Problem retreiving data from: {url}, check connection"))
                .text()
                .expect(&format!("Problem converting data from: {url} to text"))
        }
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

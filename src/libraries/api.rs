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

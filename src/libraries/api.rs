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
        self.json = ureq::get(&self.url)
        .call()
        .expect("Problem retrieving data")
        .into_string()
        .expect("Problem converting to string");
        return self;
    }
}


const API_BASE: &str = "https://www.googleapis.com/youtube/v3";

pub struct Youtube {
    api_key: String,
}

impl Youtube {
    fn search(&self, query: &str) {
        let request_string = format!("{}/search?q={}&maxResults=25&part=snippet&key={}", API_BASE, query, self.api_key);
    }
}
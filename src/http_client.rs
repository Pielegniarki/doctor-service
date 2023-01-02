pub struct HttpClient {
    http: reqwest::Client
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new()
        }
    }

    pub fn post(&self) {
        todo!();
    }
}
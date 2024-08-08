use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct RequestClient {
    client: Client,
    api_key: String,
}

impl RequestClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        RequestClient { client, api_key }
    }

    pub async fn post(&self, url: &str, body: &Value) -> Result<reqwest::Response, Box<dyn Error>> {
        let response = self
            .client
            .post(url)
            .header("content-type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(body)
            .send()
            .await?;
        Ok(response)
    }
}
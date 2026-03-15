use reqwest::{Method, header};
use serde::de::DeserializeOwned;

use crate::{Result, error::Error};

pub struct Client {
    pub base_url: String,
    pub api_key: String,
    pub(crate) http: reqwest::Client,
}

impl Client {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(Error::EmptyApiKey);
        }

        Ok(Self {
            base_url: base_url.into(),
            api_key,
            http: reqwest::Client::new(),
        })
    }

    pub(crate) async fn http_json_v1<T, F>(
        &self,
        method: Method,
        path: &str,
        configure: F,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    {
        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let request = configure(self.http.request(method, url))
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key));

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status, body });
        }

        Ok(response.json().await?)
    }
}

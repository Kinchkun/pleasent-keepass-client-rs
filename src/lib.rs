mod app;
pub mod settings;
mod timed_cache;
mod types;

use log::*;
use serde::Deserialize;
use std::error::Error;
use url::Url;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct PleasantPasswordServerClient {
    url: Url,
    login: String,
    password: String,
    client: reqwest::Client,
    cache: timed_cache::TimedCache,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

impl PleasantPasswordServerClient {
    pub fn new(url: Url, login: String, password: String) -> Result<Self> {
        Ok(PleasantPasswordServerClient {
            url,
            login,
            password,
            client: reqwest::Client::new(),
            cache: timed_cache::TimedCache::open(app::app_file(
                "pleasant_password_client",
                "cache",
            )?)?,
        })
    }

    pub async fn entry_password(&self, entry_id: &str) -> Result<Option<String>> {
        if let Some(password) = self.cache.get(entry_id)? {
            info!("Found password in cache");
            return Ok(Some(password));
        }

        let access_token = self.login().await?;
        info!("Login successful");
        let target = self
            .url
            .join(format!("api/v5/rest/Entries/{}/password", entry_id).as_str())
            .expect("invalid entry url. Maybe wrong entry id?");

        let response = self
            .client
            .get(target)
            .bearer_auth(access_token)
            .send()
            .await?
            .text()
            .await?;

        debug!("{}", response);

        self.cache.put(entry_id, response.as_str(), 60 * 60 * 24);
        Ok(Some(response))
    }

    async fn login(&self) -> Result<String> {
        info!("Login into {}", self.url);
        let cached_access_key = self.cache.get("ACCESS_TOKEN")?;
        if let Some(access_key) = cached_access_key {
            info!("A cached access key was found.");
            return Ok(access_key);
        } else {
            info!("No access key cached. Logging in");
        }

        let params = [
            ("grant_type", "password"),
            ("username", self.login.as_str()),
            ("password", self.password.as_str()),
        ];

        let target = self
            .url
            .join("/OAuth2/token")
            .expect("Invalid oauth2 path.");

        let response: TokenResponse = self
            .client
            .post(target)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        self.cache
            .put("ACCESS_TOKEN", response.access_token.as_str(), 1036799);
        return Ok(response.access_token);
    }
}

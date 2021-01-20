mod app;
mod db;
mod http_client;
pub mod settings;
mod timed_cache;
mod types;

use crate::http_client::HttpClient;
use log::*;
use serde::Deserialize;
use std::error::Error;
use url::Url;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct PleasantPasswordServerClient {
    login: String,
    password: String,
    http_client: HttpClient,
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
            login,
            password,
            http_client: HttpClient::new(url),
            cache: timed_cache::TimedCache::open(app::app_file(
                "pleasant_password_client",
                "cache",
            )?)?,
        })
    }

    pub async fn list_entries(&self) -> Result<String> {
        let access_token = self.login().await?;
        Ok(self.http_client.get_tree(access_token).await?.text())
    }

    pub async fn entry_password(&self, entry_id: &str) -> Result<Option<String>> {
        if let Some(password) = self.cache.get(entry_id)? {
            info!("Found password in cache");
            return Ok(Some(password));
        }

        let access_token = self.login().await?;
        info!("Login successful");
        let response = self
            .http_client
            .get_entry_password(access_token, entry_id)
            .await?
            .text()
            .await?;

        debug!("{}", response);

        // pleasants returns the password quoted, for some reasons. Maybe a json string?
        let response = response.trim_matches('"').to_string();

        self.cache.put(entry_id, response.as_str(), 60 * 60 * 24);
        Ok(Some(response))
    }

    async fn login(&self) -> Result<String> {
        info!("Login in");
        let cached_access_key = self.cache.get("ACCESS_TOKEN")?;
        if let Some(access_key) = cached_access_key {
            info!("A cached access key was found.");
            return Ok(access_key);
        } else {
            info!("No access key cached. Logging in");
        }

        let response: TokenResponse = self
            .http_client
            .login(self.login.as_ref(), self.password.as_ref())
            .await?
            .json()
            .await?;

        self.cache
            .put("ACCESS_TOKEN", response.access_token.as_str(), 1036799);
        return Ok(response.access_token);
    }
}

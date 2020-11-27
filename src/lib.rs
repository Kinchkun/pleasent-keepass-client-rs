use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use url::Url;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct PleasantPasswordServerClient {
    url: Url,
    login: String,
    password: String,
    client: reqwest::Client,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

impl PleasantPasswordServerClient {
    pub fn new(url: Url, login: String, password: String) -> Self {
        PleasantPasswordServerClient {
            url,
            login,
            password,
            client: reqwest::Client::new(),
        }
    }

    pub async fn entry_password(&self, entry_id: &str) -> Result<Option<String>> {
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

        println!("{}", response);

        Ok(None)
    }

    async fn login(&self) -> Result<String> {
        info!("Login into {}", self.url);
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
        return Ok(response.access_token);
    }
}

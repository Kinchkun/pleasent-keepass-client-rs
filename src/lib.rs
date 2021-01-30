pub mod app;
mod db;
mod error;
pub mod model;
mod rest;
pub mod settings;
mod timed_cache;
mod types;

pub use error::{Kind, PleasantError};
pub use rest::rest_client::RestClientBuilder;

use crate::db::db_types::Folder;
use crate::model::{Credentials, PleasantPasswordModel};
use crate::rest::rest_client::RestClient;
use crate::types::PleasantResult;
use lazy_init::Lazy;
use log::*;
use rusqlite::Connection;
use serde::Deserialize;
use std::error::Error;
use url::Url;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct PleasantPasswordServerClient {
    login: String,
    password: String,
    rest_client: RestClient,
    database_url: String,
    database_connection: Lazy<Result<Connection>>, // cache: timed_cache::TimedCache,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

impl PleasantPasswordServerClient {
    pub fn new(
        rest_client: RestClient,
        login: String,
        password: String,
        database_url: String,
    ) -> Result<Self> {
        Ok(PleasantPasswordServerClient {
            login,
            password,
            rest_client,
            database_url,
            database_connection: Lazy::new(),
            // cache: timed_cache::TimedCache::open(app::app_file(
            //     "pleasant_password_client",
            //     "cache",
            // )?)?,
        })
    }

    pub async fn check(&self) -> PleasantResult<()> {
        info!("Checking configuration");
        match self.login().await {
            Ok(token) => Ok(token),
            Err(error) => Err(PleasantError {
                kind: Kind::WrongCredentials,
                message: "Server denied the provided credentials".to_string(),
                context: "logging in".to_string(),
                hint: None,
            }),
        }?;
        Ok(())
    }

    pub fn query(&self, query: &str) -> Result<Vec<Credentials>> {
        let connection = self.open_database_connection()?;
        let model = PleasantPasswordModel::new(connection)?;
        model.query_for_credentials(query)
    }

    pub async fn sync(&self) -> Result<()> {
        info!("Syncing local with remote database");
        let connection = self.open_database_connection()?;
        let model = PleasantPasswordModel::new(connection)?;
        let root_folder = self.list_entries().await?;
        model.add_root_folder(root_folder)
    }

    pub async fn list_entries(&self) -> Result<Folder> {
        info!("Fetching all credentials entries");
        let access_token = self.login().await?;
        let root_folder: Folder = self
            .rest_client
            .get_resource(access_token, "/api/v5/rest/folders")
            .await
            .expect("Error while getting root folder")
            .expect("Error while reading root folder");
        debug!("Successfully got root folder");
        Ok(root_folder)
    }

    pub async fn entry_password(&self, entry_id: &str) -> Result<Option<String>> {
        info!("Requesting a password for entry {}", entry_id);
        // if let Some(password) = self.cache.get(entry_id)? {
        //     info!("Found password in cache");
        //     return Ok(Some(password));
        // }

        let access_token = self.login().await?;

        let password: Option<String> = self
            .rest_client
            .get_resource(
                access_token,
                format!("api/v5/rest/Entries/{}/password", entry_id),
            )
            .await
            .expect("got error while getting password entry");
        debug!(
            "Password call successfully. Got {} ",
            if password.is_some() { "some" } else { "none" }
        );
        Ok(password)
    }

    async fn login(&self) -> Result<String> {
        info!("Login in");
        // let cached_access_key = self.cache.get("ACCESS_TOKEN")?;
        // if let Some(access_key) = cached_access_key {
        //     info!("A cached access key was found.");
        //     return Ok(access_key);
        // } else {
        //     info!("No access key cached. Logging in");
        // }

        let token_response = self
            .rest_client
            .request_access_token(&self.login, &self.password)
            .await
            .expect("Got oautherror while logging in");

        // self.cache
        //     .put("ACCESS_TOKEN", response.access_token.as_str(), 1036799);
        debug!("Login successful");
        return Ok(token_response.access_token);
    }

    fn open_database_connection(&self) -> Result<&Connection> {
        let connection = self
            .database_connection
            .get_or_create(|| {
                let con = if self.database_url == ":mem:" {
                    Connection::open_in_memory()
                } else {
                    Connection::open(&self.database_url)
                };
                Ok(con?)
            })
            .as_ref()
            .expect("could not open connection");
        Ok(connection)
    }
}

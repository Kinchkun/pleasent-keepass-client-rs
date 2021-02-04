use std::error::Error;

use lazy_init::Lazy;
use log::*;
use rusqlite::Connection;
use serde::Deserialize;
use url::Url;

pub use error::{Kind, PleasantError};
use model::http_types::Folder;
pub use rest::rest_client::RestClientBuilder;

use crate::error::ResultExt;
use crate::rest::rest_client::RestClient;
pub use crate::types::PleasantResult;

pub mod app;
pub mod client;
mod error;
pub mod model;
mod rest;
pub mod settings;
mod timed_cache;
mod types;

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

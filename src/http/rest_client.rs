use crate::http::rest_client::OAuthError::NetworkError;
use chrono::{DateTime, Utc};
use log::*;
use reqwest::{Client, ClientBuilder, Error, IntoUrl, RequestBuilder, Response, StatusCode};
use serde::Deserialize;
use std::alloc::dealloc;
use url::Url;

pub struct RestClient {
    base_url: Url,
    client: Client,
}

pub struct RestClientBuilder {
    base_url: Url,
    client: ClientBuilder,
}

pub type RestResult<T, E = RestError> = std::result::Result<T, E>;
pub struct RestError;

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct OAuthErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug)]
pub enum OAuthError {
    InvalidGrant,
    UnsupportedGrantType,
    UnsupportedTokenType {
        token_type: String,
    },
    ProtocolError {
        message: String,
    },
    ServerUnavailable,
    NetworkError {
        message: String,
        cause: std::boxed::Box<dyn std::error::Error>,
    },
}

impl PartialEq for OAuthError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OAuthError::InvalidGrant, OAuthError::InvalidGrant)
            | (OAuthError::ServerUnavailable, OAuthError::ServerUnavailable)
            | (OAuthError::UnsupportedGrantType, OAuthError::UnsupportedGrantType) => true,
            (
                OAuthError::UnsupportedTokenType { token_type },
                OAuthError::UnsupportedTokenType {
                    token_type: other_token_type,
                },
            ) => token_type == other_token_type,
            (
                OAuthError::ProtocolError { message },
                OAuthError::ProtocolError {
                    message: other_message,
                },
            ) => message == other_message,
            (
                OAuthError::NetworkError { message, cause },
                OAuthError::NetworkError {
                    message: other_message,
                    cause: other_cause,
                },
            ) => message == other_message && cause.to_string() == other_cause.to_string(),
            _ => false,
        }
    }
}

impl From<reqwest::Error> for OAuthError {
    fn from(error: reqwest::Error) -> Self {
        NetworkError {
            message: "An unhandled network error occurred".to_string(),
            cause: Box::new((error)),
        }
    }
}

impl RestClientBuilder {
    pub fn new<U: IntoUrl>(u: U) -> Self {
        RestClientBuilder {
            base_url: u.into_url().expect("invalid url"),
            client: ClientBuilder::new(),
        }
    }

    pub fn proxy<U: IntoUrl>(&mut self, u: Option<U>) -> &mut Self {
        todo!()
    }

    pub fn build(self) -> RestClient {
        RestClient {
            base_url: self.base_url,
            client: self.client.build().expect("could not build client"),
        }
    }
}
impl RestClient {
    pub async fn request_access_token(
        &self,
        login: &str,
        password: &str,
    ) -> Result<AccessToken, OAuthError> {
        debug!(
            "Request new access token from {} as {} using password grant",
            &self.base_url, login
        );

        let params = [
            ("grant_type", "password"),
            ("username", login),
            ("password", password),
        ];

        let response = self
            .post("/OAuth2/token")
            .form(&params)
            .send()
            .await
            .expect("error while making token call");

        trace!("Got server response. Status code: {}", response.status());
        match response.status() {
            StatusCode::OK => Self::read_token_response(response).await,
            StatusCode::BAD_REQUEST => Self::read_oauth_error(response).await,
            StatusCode::INTERNAL_SERVER_ERROR => Err(OAuthError::ServerUnavailable),
            _ => Err(OAuthError::ProtocolError {
                message: format!(
                    "Got invalid status code for token request. Status code {}. Body: \n {}",
                    response.status(),
                    response.text().await?
                ),
            }),
        }
    }

    pub async fn get_resource<T>(&self) -> RestResult<T> {
        todo!()
    }

    pub async fn put_resource<T>(&self) -> RestResult<T> {
        todo!()
    }

    async fn read_token_response(response: Response) -> Result<AccessToken, OAuthError> {
        let access_token: AccessToken = response.json().await?;
        debug!("Got an access token");
        if access_token.token_type != "bearer" {
            warn!("Got an unsupported token type: {}", access_token.token_type);
            return Err(OAuthError::UnsupportedTokenType {
                token_type: access_token.token_type,
            });
        }
        Ok(access_token)
    }

    async fn read_oauth_error(response: Response) -> Result<AccessToken, OAuthError> {
        let error: OAuthErrorResponse = response.json().await?;
        debug!("Got an error response: {:#?}", error);
        Err(match error.error.as_str() {
            "invalid_grant" => OAuthError::InvalidGrant,
            _ => OAuthError::ProtocolError {
                message: format!("Got invalid token error response: {:#?}", error),
            },
        })
    }

    fn post(&self, path: &str) -> RequestBuilder {
        let target = self.target(path);
        self.client.post(target)
    }

    fn get(&self, path: &str) -> RequestBuilder {
        let target = self.target(path);
        self.client.get(target)
    }

    fn target(&self, path: &str) -> Url {
        self.base_url.join(path).expect("Not an url")
    }
}

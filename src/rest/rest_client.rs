use std::alloc::dealloc;

use chrono::{DateTime, Utc};
use log::*;
use reqwest::{Client, ClientBuilder, Error, IntoUrl, RequestBuilder, Response, StatusCode};
use serde::Deserialize;
use url::Url;

use crate::rest::rest_error::OAuthError;
use crate::rest::rest_error::OAuthError::NetworkError;
use serde::de::DeserializeOwned;

pub struct RestClient {
    base_url: Url,
    client: Client,
}

pub struct RestClientBuilder {
    base_url: Url,
    client: ClientBuilder,
}

pub type RestResult<T, E = OAuthError> = std::result::Result<T, E>;

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

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.access_token
    }
}

impl RestClientBuilder {
    pub fn new<U: IntoUrl>(u: U) -> Self {
        RestClientBuilder {
            base_url: u.into_url().expect("invalid url"),
            client: ClientBuilder::new(),
        }
    }

    pub fn proxy<U: IntoUrl>(mut self, u: Option<U>) -> Self {
        if let Some(proxy_scheme) = u {
            let proxy = reqwest::Proxy::all(proxy_scheme).expect("Invalid proxy scheme");
            self.client = self.client.proxy(proxy);
        }
        self
    }

    pub fn build(self) -> RestClient {
        RestClient {
            base_url: self.base_url,
            client: self.client.build().expect("could not build client"),
        }
    }
}

impl RestClient {
    fn new<U: IntoUrl>(u: U) -> Self {
        RestClientBuilder::new(u).build()
    }

    pub async fn request_access_token<S: AsRef<str>>(
        &self,
        login: S,
        password: S,
    ) -> Result<AccessToken, OAuthError> {
        let login = login.as_ref();
        let password = password.as_ref();
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
            StatusCode::INTERNAL_SERVER_ERROR => Err(OAuthError::InternalServerError),
            _ => Err(OAuthError::ProtocolError {
                message: format!(
                    "Got invalid status code for token request. Status code {}. Body: \n {}",
                    response.status(),
                    response.text().await?
                ),
            }),
        }
    }

    pub async fn get_resource<A: AsRef<str>, P: AsRef<str>, T: DeserializeOwned>(
        &self,
        access_token: A,
        resource_path: P,
    ) -> RestResult<Option<T>> {
        let path = resource_path.as_ref();
        let response = self
            .get(path)
            .bearer_auth(access_token.as_ref())
            .send()
            .await?;

        let status = response.status();
        debug!("Got server response: {}", status);
        match status {
            StatusCode::OK => Ok(Some(response.json().await?)),
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(OAuthError::Unauthorized),
            _ => Err(OAuthError::ProtocolError {
                message: format!(
                    "While getting {} the server got an unexpected response: {}",
                    path,
                    response.text().await?
                ),
            }),
        }
    }

    pub async fn put_resource<T>(&self) -> RestResult<T> {
        unimplemented!("I didn't need put operations so far. Therefore it is not yet implemented")
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

    // privates

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

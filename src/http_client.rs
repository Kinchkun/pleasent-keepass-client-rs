use crate::types::Result;
use reqwest::{RequestBuilder, Response};
use url::Url;

pub struct HttpClient {
    url: Url,
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new(url: Url, client: reqwest::Client) -> Self {
        HttpClient { url, client }
    }

    pub async fn login(&self, login: &str, password: &str) -> Result<Response> {
        let params = [
            ("grant_type", "password"),
            ("username", login),
            ("password", password),
        ];
        Ok(self.post("/OAuth2/token").form(&params).send().await?)
    }

    pub async fn get_entry_password<S: AsRef<str>>(
        &self,
        access_token: S,
        entry_id: &str,
    ) -> Result<Response> {
        Ok(self
            .get(format!("api/v5/rest/Entries/{}/password", entry_id).as_str())
            .bearer_auth(access_token.as_ref())
            .send()
            .await?)
    }

    pub async fn get_tree<S: AsRef<str>>(&self, access_token: S) -> Result<Response> {
        Ok(self
            .get("/api/v5/rest/folders")
            .bearer_auth(access_token.as_ref())
            .send()
            .await?)
    }

    fn get(&self, path: &str) -> RequestBuilder {
        let target = self.target(path);
        self.client.get(target)
    }

    fn post(&self, path: &str) -> RequestBuilder {
        let target = self.target(path);
        self.client.post(target)
    }

    fn target(&self, path: &str) -> Url {
        self.url.join(path).expect("Not an url")
    }
}

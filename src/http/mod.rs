pub mod rest_client;

#[cfg(test)]
mod tests {
    use httpmock::Method::{GET, POST};
    use httpmock::MockServer;
    use serde_json::{json, Value};

    use super::*;
    use crate::http::rest_client::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn login_with_correct_credentials_returns_a_token() {
        let token_string = "ohShoshaing4Neij2sathaiv5aihohtaizeiwieth4OnahSh3gophul6Aifeehei9isho5ohj9ha3doh5AiNahSho1Gaequ4hecu";
        let expires_in = 1000;
        let server = MockServer::start();
        let token_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/OAuth2/token")
                .body("grant_type=password&username=test_user&password=test_password");
            then.status(200)
                .header("Content-Type", "application/json;charset=UTF-8")
                .json_body(json!({
                   "access_token": token_string,
                   "token_type": "bearer",
                   "expires_in": expires_in
                }));
        });
        let target = RestClientBuilder::new(&server.base_url()).build();
        let actual = target
            .request_access_token("test_user", "test_password")
            .await;
        let expected = Ok(AccessToken {
            access_token: token_string.to_string(),
            expires_in,
            token_type: "bearer".to_string(),
        });

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn login_with_wrong_credentials_returns_invalid_grant() {
        let server = MockServer::start();
        let token_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/OAuth2/token")
                .body("grant_type=password&username=test_user&password=test_password");
            then.status(400)
                .header("Content-Type", "application/json;charset=UTF-8")
                .json_body(json!(
                {
                    "error": "invalid_grant",
                    "error_description": "The username or password is incorrect."
                }
                ));
        });
        let target = RestClientBuilder::new(&server.base_url()).build();
        let actual = target
            .request_access_token("test_user", "test_password")
            .await;
        let expected = Err(OAuthError::InvalidGrant);

        assert_eq!(actual, expected);
    }
}

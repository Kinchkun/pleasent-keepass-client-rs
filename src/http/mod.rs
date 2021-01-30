pub mod rest_client;
mod rest_error;

#[cfg(test)]
mod tests {
    use httpmock::Method::{GET, POST};
    use httpmock::MockServer;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    use crate::http::rest_client::*;
    use crate::http::rest_error::OAuthError;

    use super::*;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct TestAnimal {
        name: String,
        species: String,
        age: u64,
    }

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

    #[tokio::test]
    async fn can_request_an_resource() -> Result<(), OAuthError> {
        let token_string = "ohShoshaing4Neij2sathaiv5aihohtaizeiwieth4OnahSh3gophul6Aifeehei9isho5ohj9ha3doh5AiNahSho1Gaequ4hecu";
        let expires_in = 1000;
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .header("Authorization", format!("Bearer {}", token_string).as_str())
                .path("/api/resource/animal/bruno");
            then.status(200)
                .header("Content-Type", "application/json;charset=UTF-8")
                .json_body(json!({
                    "name": "Bruno",
                    "species": "Cat",
                    "age": 4
                }));
        });
        let target = RestClientBuilder::new(&server.base_url()).build();
        let actual: Option<TestAnimal> = target
            .get_resource(token_string, "/api/resource/animal/bruno")
            .await?;
        let expected = Some(TestAnimal {
            name: "Bruno".to_string(),
            species: "Cat".to_string(),
            age: 4,
        });
        assert_eq!(actual, expected);
        Ok(())
    }

    #[tokio::test]
    async fn handle_unauthorized() -> Result<(), OAuthError> {
        let server = MockServer::start();
        let token_string = "invalid token";
        server.mock(|when, then| {
            when.method(GET)
                .header("Authorization", format!("Bearer {}", token_string).as_str())
                .path("/api/resource/animal/bruno");
            then.status(401)
                .header("Content-Type", "application/json;charset=UTF-8");
        });
        let target = RestClientBuilder::new(&server.base_url()).build();
        let actual: Result<Option<TestAnimal>, OAuthError> = target
            .get_resource(token_string, "/api/resource/animal/bruno")
            .await;
        let expected = Err(OAuthError::Unauthorized);

        assert_eq!(actual, expected);
        Ok(())
    }

    #[tokio::test]
    async fn handle_not_found() -> Result<(), OAuthError> {
        let server = MockServer::start();
        let token_string = "invalid token";
        server.mock(|when, then| {
            when.method(GET)
                .header("Authorization", format!("Bearer {}", token_string).as_str())
                .path("/api/resource/animal/bruno");
            then.status(404)
                .header("Content-Type", "application/json;charset=UTF-8");
        });
        let target = RestClientBuilder::new(&server.base_url()).build();
        let actual: Option<TestAnimal> = target
            .get_resource(token_string, "/api/resource/animal/bruno")
            .await?;
        let expected = None;

        assert_eq!(actual, expected);
        Ok(())
    }
}

mod support;

use httpmock::Method::{GET, POST};
use httpmock::MockServer;
use pleasent_keepass_client_rs::model::{Credentials, PleasantPasswordModel};
use pleasent_keepass_client_rs::{Kind, PleasantError};
use pleasent_keepass_client_rs::{PleasantPasswordServerClient, RestClientBuilder};
use support::setup;

use pretty_assertions::assert_eq;
use reqwest::Client;

#[tokio::test]
async fn test_handling_wrong_credentials() {
    setup();
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(POST)
            .path("/OAuth2/token")
            .body("grant_type=password&username=test_user&password=test_password");
        then.status(400)
            .header("Content-Type", "application/json;charset=UTF-8")
            .body(
                r#"{
                "error": "invalid_grant",
                "error_description": "The username or password is incorrect."
            }"#,
            );
    });

    let target = PleasantPasswordServerClient::new(
        RestClientBuilder::new(&server.base_url()).build(),
        "test_user".to_string(),
        "test_password".to_string(),
        ":mem:".to_string(),
    )
    .expect("error creating client");

    let actual = target.check().await;
    let expected = Err(PleasantError {
        kind: Kind::WrongCredentials,
        message: "Server denied the provided credentials".to_string(),
        context: "logging in".to_string(),
        hint: None,
    });
    assert_eq!(actual, expected);
}

#[tokio::test]
async fn test_sync_and_query() {
    setup();

    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST)
            .path("/OAuth2/token")
            .body("grant_type=password&username=test_user&password=test_password");
        then.status(200)
            .header("Content-Type", "application/json;charset=UTF-8")
            .body(include_str!("../test_assets/token_response.json"));
    });

    server.mock(|when, then| {
        when.method(GET).path("/api/v5/rest/folders");
        then.status(200)
            .header("Content-Type", "application/json;charset=UTF-8")
            .body(include_str!("../test_assets/root_folder_response.json"));
    });

    let client = Client::new();

    let target = PleasantPasswordServerClient::new(
        RestClientBuilder::new(&server.base_url()).build(),
        "test_user".to_string(),
        "test_password".to_string(),
        ":mem:".to_string(),
    )
    .expect("error creating client");

    target.sync().await.expect("error while syncing");

    let result = target.query("mustermann").expect("could not get result");
    assert_eq!(
        result,
        vec![Credentials {
            id: "A2D7962C-FCC9-40EC-8384-9879F8EB0784".to_string(),
            folder_name: "devops".to_string(),
            name: "first child of second folder".to_string(),
            username: Some("Max@mustermann".to_string()),
            notes: Some("Some Notes".to_string())
        }]
    );
}

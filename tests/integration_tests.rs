use httpmock::Method::{GET, POST};
use httpmock::MockServer;
use pleasent_keepass_client_rs::model::{Credentials, PleasantPasswordModel};
use pleasent_keepass_client_rs::PleasantPasswordServerClient;

use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_sync_and_query() {
    pretty_env_logger::init_timed();
    let server = MockServer::start();
    let token_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/OAuth2/token")
            .body("grant_type=password&username=test_user&password=test_password");
        then.status(200)
            .header("Content-Type", "application/json;charset=UTF-8")
            .body(include_str!("../test_assets/token_response.json"));
    });

    let root_folder_mock = server.mock(|when, then| {
        when.method(GET).path("/api/v5/rest/folders");
        then.status(200)
            .header("Content-Type", "application/json;charset=UTF-8")
            .body(include_str!("../test_assets/root_folder_response.json"));
    });

    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("https://localhost:8080").expect(""))
        .build()
        .expect("");

    let target = PleasantPasswordServerClient::new(
        server.base_url().parse().unwrap(),
        client,
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

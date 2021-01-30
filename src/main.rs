use log::info;
use pleasent_keepass_client_rs::app::app_file;
use pleasent_keepass_client_rs::settings::{
    optional_url, require_secure_string, require_string, require_url,
};
use pleasent_keepass_client_rs::Result;
use pleasent_keepass_client_rs::{PleasantPasswordServerClient, RestClientBuilder};
use reqwest::Proxy;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "pleasant password client")]
enum Args {
    #[structopt(about = "retrieve the password for an entry id", alias = "pw")]
    GetPassword { entry_id: String },
    #[structopt(about = "(debug) list the entry tree")]
    Tree {},
    #[structopt(about = "download all credentials entries (without passwords)")]
    Sync {},
    #[structopt(about = "query for entries")]
    Query { query: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    let url = require_url("PLEASANT_PASSWORD_SERVER_URL");
    let http_proxy = optional_url("HTTP_PROXY");
    let login = require_string("PLEASANT_PASSWORD_SERVER_LOGIN");
    let password = require_secure_string("PLEASANT_PASSWORD_SERVER_PASSWORD");
    let database_url = app_file("pleasant_password_client", "credentials.db")?
        .to_str()
        .unwrap()
        .to_string();

    let rest_client = RestClientBuilder::new(url).proxy(http_proxy).build();

    let client = PleasantPasswordServerClient::new(
        rest_client,
        login,
        password.as_str().to_string(),
        database_url,
    )
    .expect("Could not create client");

    let args: Args = Args::from_args();

    match args {
        Args::GetPassword { entry_id } => print_password(client, entry_id).await?,
        Args::Tree {} => println!("{:#?}", client.list_entries().await?),
        Args::Sync {} => client.sync().await?,
        Args::Query { query } => print_query(client, query)?,
    };

    Ok(())
}

fn print_query(client: PleasantPasswordServerClient, query: String) -> Result<()> {
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for cred in client.query(query.as_str())?.into_iter() {
        writer.serialize(cred)?;
    }
    Ok(())
}

async fn print_password(client: PleasantPasswordServerClient, entry_id: String) -> Result<()> {
    // 94153de4-1cba-4c13-9c23-41cde415146b
    let password = client.entry_password(entry_id.as_str()).await?.unwrap();
    println!("{}", password);
    Ok(())
}

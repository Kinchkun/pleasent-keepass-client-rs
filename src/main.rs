use log::info;
use pleasent_keepass_client_rs::settings::{require_secure_string, require_string, require_url};
use pleasent_keepass_client_rs::PleasantPasswordServerClient;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(about = "pleasant password client")]
enum Args {
    #[structopt(about = "retrieve the password for an entry id", alias = "pw")]
    GetPassword { entry_id: String },
    #[structopt(about = "(debug) list the entry tree")]
    Tree {},
}

#[tokio::main]
async fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    let url = require_url("PLEASANT_PASSWORD_SERVER_URL");
    let login = require_string("PLEASANT_PASSWORD_SERVER_LOGIN");
    let password = require_secure_string("PLEASANT_PASSWORD_SERVER_PASSWORD");
    let client = PleasantPasswordServerClient::new(url, login, password.as_str().to_string())
        .expect("Could not create client");

    let args: Args = Args::from_args();

    match args {
        Args::GetPassword { entry_id } => print_password(client, entry_id).await?,
        Args::Tree => println!("{}", client.list_entries().await?),
    };

    Ok(())
}

async fn print_password(
    client: PleasantPasswordServerClient,
    entry_id: String,
) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    // 94153de4-1cba-4c13-9c23-41cde415146b
    let password = client.entry_password(entry_id.as_str()).await?.unwrap();
    println!("{}", password);
    Ok(())
}

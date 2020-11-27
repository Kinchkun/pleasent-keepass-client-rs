use pleasent_keepass_client_rs::PleasantPasswordServerClient;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(about = "pleasant password client")]
struct Args {
    entry_id: String,
}

#[tokio::main]
async fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();
    // TODO: Proper configuration option
    let login = std::env::var("PLEASANT_PASSWORD_SERVER_LOGIN")?;
    let password = std::env::var("PLEASANT_PASSWORD_SERVER_PASSWORD")?;
    let url: Url = std::env::var("PLEASANT_PASSWORD_SERVER_URL")?.parse()?;

    let args: Args = Args::from_args();

    let client = PleasantPasswordServerClient::new(url, login, password);
    client.entry_password(args.entry_id.as_str()).await?;
    // 94153de4-1cba-4c13-9c23-41cde415146b

    Ok(())
}

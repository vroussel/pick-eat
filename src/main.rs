use clap::Parser;
use pickeat_server::run;
use tokio::net::TcpListener;
use tracing::info;

mod logging;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    logging::setup(args.verbose);

    info!(
        "Starting {} v{}",
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let listener = TcpListener::bind("127.0.0.1:4242")
        .await
        .expect("Unable to bind on address");
    run(listener).await??;
    Ok(())
}

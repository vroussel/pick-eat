use clap::Parser;
use tracing::info;

mod logging;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    logging::setup(args.verbose);
    info!(
        "Starting {} v{}",
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    Ok(())
}

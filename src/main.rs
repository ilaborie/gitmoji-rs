use clap::Parser;
use gitmoji_rs::{run, Settings};
use tracing::error;

#[tokio::main]
async fn main() {
    let settings = Settings::parse();

    if settings.verbose() {
        tracing_subscriber::fmt::init();
    }

    if let Err(err) = run(settings).await {
        error!("Oops, {err}");
    }
}

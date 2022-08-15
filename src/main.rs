use clap::Parser;
use console::Term;
use gitmoji_rs::{run, Settings};
use tracing::error;

#[tokio::main]
async fn main() {
    let settings = Settings::parse();

    if settings.verbose() {
        tracing_subscriber::fmt::init();
    }

    let term = Term::stderr();
    if let Err(err) = run(settings, &term).await {
        error!("Oops, {err}");
    }
}

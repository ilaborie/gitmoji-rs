#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::perf)]
// #![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! A [gitmoji](https://github.com/carloscuesta/gitmoji) interactive client for using gitmojis on commit messages.
//!
//! See <https://github.com/carloscuesta/gitmoji-cli>

mod cli;
mod cmd;
mod error;
mod git;
mod model;

pub use self::cli::*;
pub use self::error::*;
pub use self::model::*;

/// Running the gitmoji code
///
/// # Errors
/// If the command fail
pub async fn run(settings: Settings) -> Result<()> {
    match settings.command {
        Command::Init { default } => cmd::config(default).await,
        Command::Commit => cmd::commit().await,
        Command::Update => cmd::update_config().await,
        Command::List => cmd::list().await,
        Command::Search { text } => cmd::search(&text).await,
        #[cfg(feature = "hook")]
        Command::Hook(op) => match op {
            HookOperation::Add => cmd::create_hook().await,
            HookOperation::Remove => cmd::remove_hook().await,
            HookOperation::Apply { dest, source } => cmd::apply_hook(dest, source).await,
        },
    }
}

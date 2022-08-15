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

use std::io::stdout;

use clap::CommandFactory;
use clap_complete::generate;
use console::Term;

pub use self::cli::*;
pub use self::cmd::{
    config as gitmoji_config, read_config_or_default, read_config_or_fail, write_config,
};
pub use self::error::*;
pub use self::model::*;

/// Exit code when a configuration is require but not found
pub const EXIT_NO_CONFIG: i32 = 10;

/// Exit code when a configuration cannot been updated
pub const EXIT_CANNOT_UPDATE: i32 = 20;

/// Running the gitmoji code
///
/// # Errors
/// If the command fail
pub async fn run(settings: Settings, term: &Term) -> Result<()> {
    match settings.command {
        Command::Init { default } => gitmoji_config(default, term).await,
        Command::Commit { all, amend } => cmd::commit(all, amend, term).await,
        Command::Update { url } => cmd::update_config(url).await,
        Command::List => cmd::list().await,
        Command::Search { text } => cmd::search(&text).await,
        #[cfg(feature = "hook")]
        Command::Hook(op) => match op {
            HookOperation::Add => cmd::create_hook().await,
            HookOperation::Remove => cmd::remove_hook().await,
            HookOperation::Apply { dest, source } => cmd::apply_hook(dest, source, term).await,
        },
        Command::Completion { shell } => {
            generate(shell, &mut Settings::into_app(), "gitmoji", &mut stdout());
            Ok(())
        }
    }
}

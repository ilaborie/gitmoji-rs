use std::process::exit;

use console::Term;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use url::Url;

use crate::git::has_staged_changes;
use crate::{
    git, recovery, EmojiFormat, Error, GitmojiConfig, Result, EXIT_CANNOT_UPDATE, EXIT_NO_CONFIG,
};

mod commit;
pub use self::commit::*;

mod config;
pub use self::config::*;

#[cfg(feature = "hook")]
mod hook;

mod list;
use self::list::print_gitmojis;

mod search;
use self::search::filter;

mod update;
use self::update::update_gitmojis;

async fn get_config_or_stop() -> GitmojiConfig {
    match read_config_or_fail().await {
        Ok(config) => config,
        Err(err) => {
            warn!("Oops, cannot read config because {err}");
            eprintln!(
                "⚠️  No configuration found, try run `gitmoji init` to fetch a configuration"
            );
            exit(EXIT_NO_CONFIG)
        }
    }
}

async fn update_config_or_stop(config: GitmojiConfig) -> GitmojiConfig {
    let url = config.update_url().to_string();
    match update_gitmojis(config).await {
        Ok(config) => config,
        Err(err) => {
            warn!("Oops, cannot update the config because {err}");
            eprintln!("⚠️  Configuration not updated, maybe check the update url '{url}'");
            exit(EXIT_CANNOT_UPDATE)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitTitleDescription {
    pub title: String,
    pub description: Option<String>,
}

#[tracing::instrument(skip(term))]
async fn ask_commit_title_description(
    config: &GitmojiConfig,
    term: &Term,
) -> Result<CommitTitleDescription> {
    let CommitParams {
        gitmoji,
        scope,
        title,
        description,
    } = get_commit_params(config, term)?;

    let gitmoji = match config.format() {
        EmojiFormat::UseCode => gitmoji.code(),
        EmojiFormat::UseEmoji => gitmoji.emoji(),
    };

    let title = scope.map_or_else(
        || format!("{gitmoji} {title}"),
        |scope| format!("{gitmoji} {scope}{title}"),
    );

    let result = CommitTitleDescription { title, description };
    Ok(result)
}

/// Commit using Gitmoji
#[tracing::instrument(skip(term))]
pub async fn commit(all: bool, amend: bool, extra_args: &[String], term: &Term) -> Result<()> {
    let config = get_config_or_stop().await;

    if !amend && !has_staged_changes().await? {
        eprintln!("No change to commit");
        return Ok(());
    }

    let commit = if let Ok(Some(recovered)) = recovery::read() {
        if recovery::ask(term, &recovered).unwrap_or(false) {
            recovered
        } else {
            ask_commit_title_description(&config, term).await?
        }
    } else {
        ask_commit_title_description(&config, term).await?
    };

    // Add before commit
    let all = all || config.auto_add();

    // Commit
    let status = git::commit(
        all,
        amend,
        config.signed(),
        &commit.title,
        commit.description.as_deref(),
        extra_args,
    )
    .await?;

    if status.success() {
        if let Err(err) = recovery::clear() {
            warn!("{err}");
        }
        Ok(())
    } else {
        if let Err(err) = recovery::write(&commit) {
            warn!("{err}");
        }
        Err(Error::FailToCommit)
    }
}

/// Configure Gitmoji
///
/// # Errors
/// Returns an error if configuration creation fails or dialog interaction fails
#[tracing::instrument(skip(term))]
pub async fn config(default: bool, term: &Term) -> Result<()> {
    let config = if default {
        GitmojiConfig::default()
    } else {
        create_config(term)?
    };
    info!("Loading gitmojis from {}", config.update_url());
    update_config_or_stop(config).await;

    Ok(())
}

/// Search a gitmoji
#[tracing::instrument]
pub async fn search(text: &str) -> Result<()> {
    let config = get_config_or_stop().await;
    let result = filter(config.gitmojis(), text);
    print_gitmojis(&result);
    Ok(())
}

/// List all Gitmojis
#[tracing::instrument]
pub async fn list() -> Result<()> {
    let config = get_config_or_stop().await;
    print_gitmojis(config.gitmojis());
    Ok(())
}

/// Update the configuration with the URL
#[tracing::instrument]
pub async fn update_config(url: Option<Url>) -> Result<()> {
    let mut config = read_config_or_default().await;
    if let Some(url) = url {
        config.set_update_url(url);
    }
    let result = update_config_or_stop(config).await;
    print_gitmojis(result.gitmojis());

    Ok(())
}

/// Create hook
#[cfg(feature = "hook")]
#[tracing::instrument]
pub async fn create_hook() -> Result<()> {
    hook::create().await
}

/// Remove hook
#[tracing::instrument]
#[cfg(feature = "hook")]
pub async fn remove_hook() -> Result<()> {
    hook::remove().await
}

/// Open /dev/tty directly for interactive prompts in hook context
#[cfg(all(feature = "hook", unix))]
fn open_tty_term() -> std::io::Result<Term> {
    use std::fs::OpenOptions;

    let tty_read = OpenOptions::new().read(true).open("/dev/tty")?;
    let tty_write = OpenOptions::new().write(true).open("/dev/tty")?;

    Ok(Term::read_write_pair(tty_read, tty_write))
}

/// Apply hook
#[cfg(feature = "hook")]
#[tracing::instrument(skip(_term))]
pub async fn apply_hook(
    dest: std::path::PathBuf,
    source: Option<String>,
    _term: &Term,
) -> Result<()> {
    use tokio::io::AsyncWriteExt;

    // Open /dev/tty directly for the hook context
    // This is needed because git hooks don't have a proper terminal attached
    let term = open_tty_term().map_err(|e| Error::Hook {
        cause: format!("Cannot open /dev/tty: {e}"),
    })?;

    let config = get_config_or_stop().await;

    let CommitTitleDescription { title, description } =
        ask_commit_title_description(&config, &term).await?;

    info!("Write commit message to {dest:?} with source: {source:?}");
    let contents = tokio::fs::read(&dest).await.unwrap_or_default();
    let mut file = tokio::fs::File::create(&dest).await?;

    file.write_all(title.as_bytes()).await?;
    file.write_all(b"\n\n").await?;

    if let Some(description) = description {
        file.write_all(description.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    file.write_all(&contents).await?;

    Ok(())
}

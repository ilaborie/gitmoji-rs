use tokio::process::Command;
use tracing::{info, warn};

use crate::{git, EmojiFormat, Error, GitmojiConfig, Result};

mod commit;
mod config;
#[cfg(feature = "hook")]
mod hook;
mod list;
mod search;
mod update;

pub use self::commit::*;
pub use self::config::*;
#[cfg(feature = "hook")]
pub use self::hook::*;
use self::list::print_gitmojis;
use self::search::filter;
use self::update::update_gitmojis;

#[derive(Debug, Clone)]
struct CommitTitleDescription {
    title: String,
    description: Option<String>,
}

#[tracing::instrument]
async fn ask_commit_title_description(config: &GitmojiConfig) -> Result<CommitTitleDescription> {
    let CommitParams {
        gitmoji,
        scope,
        title,
        description,
    } = get_commit_params(config)?;

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
#[tracing::instrument]
pub async fn commit() -> Result<()> {
    let config = read_config_or_fail().await?;

    let CommitTitleDescription { title, description } =
        ask_commit_title_description(&config).await?;

    // Add before commit
    if config.auto_add() {
        let status = Command::new("git").arg("add").arg(".").status().await?;
        if !status.success() {
            return Err(Error::FailToCommit);
        }
    }

    // Commit
    let status = git::commit(config.signed(), &title, description.as_deref()).await?;
    status.success().then_some(()).ok_or(Error::FailToCommit)
}

/// Configure Gitmoji
#[tracing::instrument]
pub async fn config(default: bool) -> Result<()> {
    let config = if default {
        GitmojiConfig::default()
    } else {
        create_config()?
    };
    info!("Loading gitmojis from {}", config.update_url());
    update_gitmojis(config).await?;

    Ok(())
}

/// Search a gitmoji
#[tracing::instrument]
pub async fn search(text: &str) -> Result<()> {
    let config = read_config_or_fail().await?;
    let result = filter(config.gitmojis(), text);
    print_gitmojis(&result);

    Ok(())
}

/// List all Gitmojis
#[tracing::instrument]
pub async fn list() -> Result<()> {
    match read_config_or_fail().await {
        Ok(result) => print_gitmojis(result.gitmojis()),
        Err(err) => {
            warn!("Oops, cannot read config because {err}");
            eprintln!(
                "⚠️ No configuration found, try run `gitmoji update` to fetch a configuration"
            );
        }
    }
    Ok(())
}

/// Update the configuration with the URL
#[tracing::instrument]
pub async fn update_config() -> Result<()> {
    let config = read_config_or_default().await;
    let result = update_gitmojis(config).await?;
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

/// Apply hook
#[cfg(feature = "hook")]
#[tracing::instrument]
pub async fn apply_hook(dest: std::path::PathBuf, source: Option<String>) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

    let config = read_config_or_fail().await?;

    let CommitTitleDescription { title, description } =
        ask_commit_title_description(&config).await?;

    info!("Write commit message to {dest:?} with source: {source:?}");
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(dest)
        .await?;

    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    file.seek(std::io::SeekFrom::Start(0)).await?;

    file.write_all(title.as_bytes()).await?;
    file.write_all(b"\n\n").await?;

    if let Some(description) = description {
        file.write_all(description.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    file.write_all(&contents).await?;

    Ok(())
}

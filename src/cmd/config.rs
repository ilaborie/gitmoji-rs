use std::fmt::{self, Display};
use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use directories::ProjectDirs;
use tokio::fs;
use tracing::info;

use crate::{EmojiFormat, Error, GitmojiConfig, Result, DEFAULT_URL};

#[derive(Debug, Clone)]
struct FormatItem<'d> {
    name: &'d str,
    value: EmojiFormat,
}

impl Display for FormatItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const FORMAT_ITEMS: &[FormatItem<'static>] = &[
    FormatItem {
        name: ":smile:",
        value: EmojiFormat::UseCode,
    },
    FormatItem {
        name: "😄",
        value: EmojiFormat::UseEmoji,
    },
];

pub fn create_config() -> Result<GitmojiConfig> {
    let theme = ColorfulTheme::default();
    let auto_add = Confirm::with_theme(&theme)
        .with_prompt(r#"Enable automatic "git add .""#)
        .default(false)
        .interact()?;
    let format_idx = Select::with_theme(&theme)
        .with_prompt("Select how emojis should be used in commits")
        .default(0)
        .items(FORMAT_ITEMS)
        .interact()?;
    let format = FORMAT_ITEMS[format_idx].value;
    let signed = Confirm::with_theme(&theme)
        .with_prompt("Enable signed commits")
        .default(false)
        .interact()?;
    let scope = Confirm::with_theme(&theme)
        .with_prompt("Enable scope prompt")
        .default(false)
        .interact()?;
    let update_url = Input::with_theme(&theme)
        .with_prompt("Set gitmojis api url")
        .default(DEFAULT_URL.to_string())
        .validate_with(validate_url)
        .interact_text()?
        .parse()?;

    let config = GitmojiConfig::new(auto_add, format, signed, scope, update_url);
    Ok(config)
}

#[allow(clippy::ptr_arg)]
fn validate_url(s: &String) -> Result<()> {
    let _url = s.parse::<url::Url>()?;
    Ok(())
}

pub async fn get_config_file() -> Result<PathBuf> {
    let project_dir =
        ProjectDirs::from("com.github", "ilaborie", "gitmoji-rs").ok_or_else(|| {
            Error::CannotGetProjectConfigFile("cannot define project dir".to_string())
        })?;

    let config_dir = project_dir.config_dir();

    fs::create_dir_all(config_dir)
        .await
        .map_err(|err| Error::CannotGetProjectConfigFile(err.to_string()))?;

    let mut config_file = config_dir.to_path_buf();
    config_file.push("gitmojis.toml");

    Ok(config_file)
}

async fn read_config() -> Result<GitmojiConfig> {
    let config_file = get_config_file().await?;
    info!("Read config file {config_file:?}");
    let bytes = fs::read(config_file).await?;
    let result = toml::from_slice(&bytes)?;

    Ok(result)
}

pub async fn read_config_or_fail() -> Result<GitmojiConfig> {
    read_config().await.map_err(|_| Error::MissingConfigFile)
}

pub async fn read_config_or_default() -> GitmojiConfig {
    read_config().await.unwrap_or_default()
}

pub async fn write_config(config: &GitmojiConfig) -> Result<()> {
    let config_file = get_config_file().await?;
    let contents = toml::to_string_pretty(config)?;
    info!("Update config file {config_file:?}");
    fs::write(config_file, contents).await?;
    Ok(())
}

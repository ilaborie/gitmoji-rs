use std::fmt::{self, Display};
use std::path::PathBuf;

use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use directories::ProjectDirs;
use tokio::fs;
use tracing::info;

use crate::{EmojiFormat, Error, GitmojiConfig, Result, DEFAULT_URL};

const CONFIG_FILE: &str = "gitmojis.toml";
const DIR_QUALIFIER: &str = "com.github";
const DIR_ORGANIZATION: &str = "ilaborie";
const DIR_APPLICATION: &str = "gitmoji-rs";
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

pub fn create_config(term: &Term) -> Result<GitmojiConfig> {
    let theme = ColorfulTheme::default();
    let auto_add = Confirm::with_theme(&theme)
        .with_prompt(r#"Enable automatic "git add .""#)
        .default(false)
        .interact_on(term)?;

    let format_idx = Select::with_theme(&theme)
        .with_prompt("Select how emojis should be used in commits")
        .default(0)
        .items(FORMAT_ITEMS)
        .interact_on(term)?;
    let format = FORMAT_ITEMS[format_idx].value;

    let signed = Confirm::with_theme(&theme)
        .with_prompt("Enable signed commits")
        .default(false)
        .interact_on(term)?;

    let scope = Confirm::with_theme(&theme)
        .with_prompt("Enable scope prompt")
        .default(false)
        .interact_on(term)?;

    let update_url = Input::with_theme(&theme)
        .with_prompt("Set gitmojis api url")
        .default(DEFAULT_URL.to_string())
        .validate_with(validate_url)
        .interact_text_on(term)?
        .parse()?;

    let config = GitmojiConfig::new(auto_add, format, signed, scope, update_url);
    Ok(config)
}

#[allow(clippy::ptr_arg)]
fn validate_url(s: &String) -> Result<()> {
    let _url = s.parse::<url::Url>()?;
    Ok(())
}

/// Get the configuration file
///
/// # Errors
/// Fail if we cannot create the parent directory
pub async fn get_config_file() -> Result<PathBuf> {
    let project_dir = ProjectDirs::from(DIR_QUALIFIER, DIR_ORGANIZATION, DIR_APPLICATION)
        .ok_or_else(|| {
            Error::CannotGetProjectConfigFile("cannot define project dir".to_string())
        })?;

    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir)
        .await
        .map_err(|err| Error::CannotGetProjectConfigFile(err.to_string()))?;

    let mut config_file = config_dir.to_path_buf();
    config_file.push(CONFIG_FILE);

    Ok(config_file)
}

async fn read_config() -> Result<GitmojiConfig> {
    let config_file = get_config_file().await?;
    info!("Read config file {config_file:?}");
    let bytes = fs::read(config_file).await?;
    let result = toml::from_slice(&bytes)?;

    Ok(result)
}

/// Read the user config file
///
/// # Errors
/// Fail when the config file is not found
pub async fn read_config_or_fail() -> Result<GitmojiConfig> {
    read_config().await.map_err(|_| Error::MissingConfigFile)
}

/// Read the user config file, if the file does not exists, return the default configuration
pub async fn read_config_or_default() -> GitmojiConfig {
    read_config().await.unwrap_or_default()
}

/// Write config
///
/// # Errors
/// Fail when I/O trouble to get or write the file
/// Might fail during serialization of config
pub async fn write_config(config: &GitmojiConfig) -> Result<()> {
    let config_file = get_config_file().await?;
    let contents = toml::to_string_pretty(config)?;
    info!("Update config file {config_file:?}");
    fs::write(config_file, contents).await?;
    Ok(())
}

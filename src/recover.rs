use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::cmd::CommitTitleDescription;
use std::{
    fs,
    io::{Error, ErrorKind::NotFound},
};

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("Could not {action} recovery file: {source}")]
pub struct RecoveryError {
    // The source error
    #[error(source)]
    source: Error,

    action: String,
}

type Result<T> = std::result::Result<T, RecoveryError>;

pub fn ask_for_recovery(
    term: &Term,
    recovered: CommitTitleDescription,
) -> std::result::Result<bool, dialoguer::Error> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Last commit failed. Reuse recovered commit message “{}” ?",
            recovered.title
        ))
        .default(true)
        .interact_on(term)?)
}

pub fn write_recovery_file(params: CommitTitleDescription) -> Result<()> {
    let path = directories::BaseDirs::new()
        .ok_or(RecoveryError {
            source: Error::new(NotFound, "Base directories not found"),
            action: "write".to_string(),
        })?
        .cache_dir()
        .join("gitmoji-rs")
        .join("lastmessage.json");

    if !path.exists() {
        let dir = path.parent().ok_or(RecoveryError {
            source: Error::new(NotFound, "path to file is has no parent"),
            action: "create directory for".to_string(),
        })?;

        fs::create_dir_all(dir).expect("Failed to create recovery directory");
    }

    let content = serde_json::to_string(&params).map_err(|source| RecoveryError {
        source: source.into(),
        action: "serialize".to_string(),
    })?;

    fs::write(path, content).map_err(|source| RecoveryError {
        source,
        action: "write".to_string(),
    })?;

    Ok(())
}

pub fn read_recovery_file() -> Result<Option<CommitTitleDescription>> {
    let path = directories::BaseDirs::new()
        .ok_or(RecoveryError {
            source: Error::new(NotFound, "Base directories not found"),
            action: "read".to_string(),
        })?
        .cache_dir()
        .join("gitmoji-rs")
        .join("lastmessage.json");

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path).map_err(|source| RecoveryError {
        source,
        action: "read".to_string(),
    })?;

    let params: CommitTitleDescription =
        serde_json::from_str(&content).map_err(|source| RecoveryError {
            source: source.into(),
            action: "deserialize".to_string(),
        })?;

    Ok(Some(params))
}

pub fn clear_recovery_file() -> Result<()> {
    let path = directories::BaseDirs::new()
        .ok_or(RecoveryError {
            source: Error::new(NotFound, "Base directories not found"),
            action: "clear".to_string(),
        })?
        .cache_dir()
        .join("gitmoji-rs")
        .join("lastmessage.json");

    if path.exists() {
        fs::remove_file(path).map_err(|source| RecoveryError {
            source,
            action: "remove".to_string(),
        })?;
    }

    Ok(())
}

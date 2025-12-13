use std::fs;
use std::path::PathBuf;

use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use crate::cmd::CommitTitleDescription;

const RECOVERY_FILE: &str = "lastmessage.toml";
const RECOVERY_DIR: &str = "gitmoji-rs";

/// Recovery file errors
#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
#[non_exhaustive]
pub enum RecoveryError {
    /// Cannot find base directories
    #[display("Cannot find base directories for recovery file")]
    #[from(ignore)]
    NoBaseDirectories,

    /// Cannot read recovery file
    #[display("Cannot read recovery file: {_0}")]
    Read(std::io::Error),

    /// Cannot write recovery file
    #[display("Cannot write recovery file: {_0}")]
    #[from(ignore)]
    Write(std::io::Error),

    /// Cannot serialize recovery data
    #[display("Cannot serialize recovery data: {_0}")]
    Serialize(toml_edit::ser::Error),

    /// Cannot deserialize recovery data
    #[display("Cannot deserialize recovery data: {_0}")]
    Deserialize(toml_edit::de::Error),
}

type Result<T> = std::result::Result<T, RecoveryError>;

/// Get the path to the recovery file
fn recovery_file_path() -> Result<PathBuf> {
    let base_dirs = directories::BaseDirs::new().ok_or(RecoveryError::NoBaseDirectories)?;
    let path = base_dirs.cache_dir().join(RECOVERY_DIR).join(RECOVERY_FILE);
    Ok(path)
}

/// Ask the user if they want to recover the last commit message
pub fn ask(
    term: &Term,
    recovered: &CommitTitleDescription,
) -> std::result::Result<bool, dialoguer::Error> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Last commit failed. Reuse recovered commit message \"{}\"?",
            recovered.title
        ))
        .default(true)
        .interact_on(term)
}

/// Write the commit message to the recovery file
pub fn write(params: &CommitTitleDescription) -> Result<()> {
    let path = recovery_file_path()?;

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(RecoveryError::Write)?;
    }

    let content = toml_edit::ser::to_string(params)?;
    fs::write(path, content).map_err(RecoveryError::Write)?;

    Ok(())
}

/// Read the commit message from the recovery file
pub fn read() -> Result<Option<CommitTitleDescription>> {
    let path = recovery_file_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)?;
    let params = toml_edit::de::from_str(&content)?;

    Ok(Some(params))
}

/// Clear the recovery file
pub fn clear() -> Result<()> {
    let path = recovery_file_path()?;

    if path.exists() {
        fs::remove_file(path)?;
    }

    Ok(())
}

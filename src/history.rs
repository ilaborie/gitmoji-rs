use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use dialoguer::{BasicHistory, History};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::git::get_git_dir_sync;

const HISTORY_DIR: &str = "history";
const MAX_HISTORY_ENTRIES: usize = 20;

#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
#[non_exhaustive]
pub enum HistoryError {
    #[display("Cannot find base directories")]
    #[from(ignore)]
    NoBaseDirectories,

    #[display("Cannot get git directory: {_0}")]
    GitDir(crate::git::GitCommandError),

    #[display("Cannot read history: {_0}")]
    Read(std::io::Error),

    #[display("Cannot write history: {_0}")]
    #[from(ignore)]
    Write(std::io::Error),

    #[display("Cannot serialize history: {_0}")]
    Serialize(toml_edit::ser::Error),

    #[display("Cannot deserialize history: {_0}")]
    Deserialize(toml_edit::de::Error),
}

type Result<T> = std::result::Result<T, HistoryError>;

#[derive(Debug, Serialize, Deserialize, Default)]
struct HistoryData {
    entries: Vec<String>,
}

/// Scope history with persistence
pub struct ScopeHistory {
    inner: BasicHistory,
    file_path: PathBuf,
}

impl Default for ScopeHistory {
    fn default() -> Self {
        Self {
            inner: BasicHistory::new()
                .max_entries(MAX_HISTORY_ENTRIES)
                .no_duplicates(true),
            file_path: PathBuf::new(),
        }
    }
}

impl ScopeHistory {
    /// Load history for the current repository
    ///
    /// # Errors
    /// Returns an error if:
    /// - Base directories cannot be determined
    /// - Git directory cannot be found (not in a git repository)
    /// - History file exists but cannot be read or parsed
    pub fn load() -> Result<Self> {
        let file_path = Self::history_file_path()?;
        let mut inner = BasicHistory::new()
            .max_entries(MAX_HISTORY_ENTRIES)
            .no_duplicates(true);

        if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            let data: HistoryData = toml_edit::de::from_str(&content)?;
            // Load in reverse so most recent is first
            for entry in data.entries.into_iter().rev() {
                inner.write(&entry);
            }
        }

        Ok(Self { inner, file_path })
    }

    fn save(&self) -> Result<()> {
        if let Some(dir) = self.file_path.parent() {
            fs::create_dir_all(dir).map_err(HistoryError::Write)?;
        }
        // Collect entries from history
        let mut entries = Vec::new();
        let mut pos = 0;
        while let Some(entry) = <BasicHistory as History<String>>::read(&self.inner, pos) {
            entries.push(entry);
            pos += 1;
        }
        let data = HistoryData { entries };
        let content = toml_edit::ser::to_string(&data)?;
        fs::write(&self.file_path, content).map_err(HistoryError::Write)?;
        Ok(())
    }

    fn history_file_path() -> Result<PathBuf> {
        let base_dirs = directories::BaseDirs::new().ok_or(HistoryError::NoBaseDirectories)?;
        let repo_hash = Self::get_repo_hash()?;
        let path = base_dirs
            .cache_dir()
            .join("gitmoji-rs")
            .join(HISTORY_DIR)
            .join(format!("{repo_hash}.toml"));
        Ok(path)
    }

    fn get_repo_hash() -> Result<String> {
        let git_dir = get_git_dir_sync()?;
        let mut hasher = DefaultHasher::new();
        git_dir.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }
}

impl History<String> for ScopeHistory {
    fn read(&self, pos: usize) -> Option<String> {
        <BasicHistory as History<String>>::read(&self.inner, pos)
    }

    fn write(&mut self, val: &String) {
        <BasicHistory as History<String>>::write(&mut self.inner, val);
        if let Err(err) = self.save() {
            warn!("Failed to save scope history: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_and_deserialize_history_data() {
        let data = HistoryData {
            entries: vec!["api".to_string(), "cli".to_string(), "tests".to_string()],
        };

        let toml = toml_edit::ser::to_string(&data).expect("serialize");
        let result: HistoryData = toml_edit::de::from_str(&toml).expect("deserialize");

        assert_eq!(result.entries, data.entries);
    }
}

use std::process::ExitStatus;

use tokio::process::Command;

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("Fail to run `{command}` because {source}")]
pub struct GitCommandError {
    /// The source error
    #[error(source)]
    source: std::io::Error,

    /// The command
    command: String,
}

type Result<T> = std::result::Result<T, GitCommandError>;

pub(crate) async fn commit(
    all: bool,
    amend: bool,
    signed: bool,
    commit_title: &str,
    description: Option<&str>,
) -> Result<ExitStatus> {
    let mut args = vec!["commit"];
    if all {
        args.push("--all");
    }
    if amend {
        args.push("--amend");
    }
    if signed {
        args.push("-S");
    }
    args.push("-m");
    args.push(commit_title);
    if let Some(description) = description {
        args.push("-m");
        args.push(description);
    }
    let status = Command::new("git")
        .args(&args)
        .status()
        .await
        .map_err(|source| GitCommandError {
            source,
            command: format!("git {}", args.join(" ")),
        })?;

    Ok(status)
}

pub(crate) async fn get_config_value(config_key: &str) -> Result<String> {
    let args = ["config", "--get", config_key];
    let output = Command::new("git")
        .args(args)
        .output()
        .await
        .map_err(|source| GitCommandError {
            source,
            command: format!("git {}", args.join(" ")),
        })?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(result)
}

pub(crate) async fn has_staged_changes() -> Result<bool> {
    let args = ["status", "--porcelain"];
    let output = Command::new("git")
        .args(args)
        .output()
        .await
        .map_err(|source| GitCommandError {
            source,
            command: format!("git {}", args.join(" ")),
        })?;

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let first_char = line.chars().next().unwrap_or_default();
        if first_char != ' ' {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(feature = "hook")]
pub(crate) async fn get_git_dir() -> Result<std::path::PathBuf> {
    let args = ["rev-parse", "--absolute-git-dir"];
    let output = Command::new("git")
        .args(args)
        .output()
        .await
        .map_err(|source| GitCommandError {
            source,
            command: format!("git {}", args.join(" ")),
        })?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let result = std::path::PathBuf::from(result);
    Ok(result)
}

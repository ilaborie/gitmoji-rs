use std::process::ExitStatus;

use tokio::process::Command;

use crate::Result;

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
    let status = Command::new("git").args(&args).status().await?;

    Ok(status)
}

#[cfg(feature = "hook")]
pub(crate) async fn get_config_value(config_key: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg(config_key)
        .output()
        .await?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(result)
}

#[cfg(feature = "hook")]
pub(crate) async fn get_git_dir() -> Result<std::path::PathBuf> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--absolute-git-dir")
        .output()
        .await?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let result = std::path::PathBuf::from(result);
    Ok(result)
}

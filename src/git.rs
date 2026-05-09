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

fn map_git_error<'a>(args: &'a [&'a str]) -> impl FnOnce(std::io::Error) -> GitCommandError + 'a {
    |source| GitCommandError {
        source,
        command: format!("git {}", args.join(" ")),
    }
}

fn build_commit_args<'a>(
    all: bool,
    amend: bool,
    signed: bool,
    commit_title: &'a str,
    description: Option<&'a str>,
    extra_args: &'a [String],
) -> Vec<&'a str> {
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
    for arg in extra_args {
        args.push(arg);
    }
    args
}

pub(crate) async fn commit(
    all: bool,
    amend: bool,
    signed: bool,
    commit_title: &str,
    description: Option<&str>,
    extra_args: &[String],
) -> Result<ExitStatus> {
    let args = build_commit_args(all, amend, signed, commit_title, description, extra_args);
    let status = Command::new("git")
        .args(&args)
        .status()
        .await
        .map_err(map_git_error(&args))?;

    Ok(status)
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use super::*;

    #[test]
    fn should_build_basic_args() {
        let args = build_commit_args(false, false, false, "feat: add thing", None, &[]);
        check!(args == vec!["commit", "-m", "feat: add thing"]);
    }

    #[test]
    fn should_build_args_with_description() {
        let args = build_commit_args(false, false, false, "feat: add thing", Some("body"), &[]);
        check!(args == vec!["commit", "-m", "feat: add thing", "-m", "body"]);
    }

    #[test]
    fn should_build_args_with_flags() {
        let args = build_commit_args(true, true, true, "feat: add thing", None, &[]);
        check!(args == vec!["commit", "--all", "--amend", "-S", "-m", "feat: add thing"]);
    }

    #[test]
    fn should_append_extra_args_at_tail() {
        let extra = vec!["--no-verify".to_string(), "--signoff".to_string()];
        let args = build_commit_args(false, false, false, "feat: add thing", None, &extra);
        check!(
            args == vec![
                "commit",
                "-m",
                "feat: add thing",
                "--no-verify",
                "--signoff"
            ]
        );
    }

    #[test]
    fn should_append_extra_args_after_all_flags() {
        let extra = vec!["--no-verify".to_string()];
        let args = build_commit_args(true, false, false, "feat: add thing", Some("body"), &extra);
        check!(
            args == vec![
                "commit",
                "--all",
                "-m",
                "feat: add thing",
                "-m",
                "body",
                "--no-verify"
            ]
        );
    }
}

pub(crate) async fn get_config_value(config_key: &str) -> Result<String> {
    let args = ["config", "--get", config_key];
    let output = Command::new("git")
        .args(args)
        .output()
        .await
        .map_err(map_git_error(&args))?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(result)
}

pub(crate) async fn has_staged_changes() -> Result<bool> {
    let args = ["status", "--porcelain"];
    let output = Command::new("git")
        .args(args)
        .output()
        .await
        .map_err(map_git_error(&args))?;

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
        .map_err(map_git_error(&args))?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let result = std::path::PathBuf::from(result);
    Ok(result)
}

pub(crate) fn get_git_dir_sync() -> Result<std::path::PathBuf> {
    let args = ["rev-parse", "--absolute-git-dir"];
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .map_err(map_git_error(&args))?;

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(std::path::PathBuf::from(result))
}

use std::fs::Permissions;
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;

use tokio::fs;
use tracing::info;

use crate::{git, Result};

const HOOK_PERMISSIONS: u32 = 0o775;
const HOOK_FILENAME: &str = "prepare-commit-msg";
const HOOK_CONTENTS: &str = include_str!("./hook.sh");

pub async fn create() -> Result<()> {
    let mut path = get_hooks_path().await?;
    path.push(HOOK_FILENAME);
    info!("Create hook in {path:?}");
    fs::write(&path, HOOK_CONTENTS).await?;
    let perm = Permissions::from_mode(HOOK_PERMISSIONS);
    fs::set_permissions(&path, perm).await?;

    Ok(())
}

pub async fn remove() -> Result<()> {
    let mut path = get_hooks_path().await?;
    path.push(HOOK_FILENAME);
    info!("Removing hook in {path:?}");
    fs::remove_file(path).await?;

    Ok(())
}

async fn get_hooks_path() -> Result<PathBuf> {
    let path = git::get_config_value("core.hooksPath").await?;
    let result = if path.is_empty() {
        let mut dir = git::get_git_dir().await?;
        dir.push("hooks");
        dir
    } else {
        PathBuf::from(path)
    };

    Ok(result)
}

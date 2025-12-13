use std::env;
use std::path::PathBuf;
use std::process::Command;

use gitmoji_rs::{write_config, Gitmoji, GitmojiConfig};

mod git;
mod isolation;

pub use self::git::*;
pub use self::isolation::*;

/// Create a test gitmoji for testing purposes
pub fn test_gitmoji() -> Gitmoji {
    Gitmoji::new(
        String::from("🧪"),
        String::from(":test_tube:"),
        Some(String::from("Test")),
        Some(String::from("A test gitmoji")),
    )
}

/// Setup a test config with a single test gitmoji
pub async fn setup_test_config() -> GitmojiConfig {
    let mut config = GitmojiConfig::default();
    config.set_gitmojis(vec![test_gitmoji()]);
    write_config(&config)
        .await
        .expect("Failed to write test config");
    config
}

// Adapted from assert_cmd
pub fn cargo_bin_command(name: &str) -> Command {
    Command::new(cargo_bin_path(name))
}

pub fn assert_cargo_bin(name: &str) -> assert_cmd::Command {
    assert_cmd::Command::new(cargo_bin_path(name))
}

fn cargo_bin_path(name: &str) -> PathBuf {
    let env_var = format!("CARGO_BIN_EXE_{name}");
    env::var_os(env_var)
        .map(|p| p.into())
        .unwrap_or_else(|| target_dir().join(format!("{name}{}", env::consts::EXE_SUFFIX)))
}

// From assert_cmd
fn target_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap()
}

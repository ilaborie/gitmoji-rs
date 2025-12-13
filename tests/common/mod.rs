use std::env;
use std::path::PathBuf;
use std::process::Command;

mod git;
mod isolation;

pub use self::git::*;
pub use self::isolation::*;

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

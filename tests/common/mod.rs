use std::env;
use std::path::PathBuf;
use std::process::Command;

mod git;
mod isolation;

pub use self::git::*;
pub use self::isolation::*;

// Adapted from assert_cmd
pub fn cargo_bin_command(name: &str) -> Command {
    let env_var = format!("CARGO_BIN_EXE_{name}");
    let path = env::var_os(env_var)
        .map(|p| p.into())
        .unwrap_or_else(|| target_dir().join(format!("{name}{}", env::consts::EXE_SUFFIX)));
    Command::new(path)
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

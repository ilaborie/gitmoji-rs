#![allow(clippy::missing_panics_doc)]

use std::env;

use assert_fs::TempDir;

#[must_use]
pub fn home_isolation() -> TempDir {
    let dir = TempDir::new().expect("Create temp dir for HOME isolation");
    let tmp_home = dir.path();
    // Set HOME and XDG vars so the `directories` crate resolves config paths inside the
    // temp dir on all platforms. Without XDG_CONFIG_HOME, Linux runners may use a
    // pre-existing XDG_CONFIG_HOME from the environment, bypassing the HOME override.
    env::set_var("HOME", tmp_home);
    env::set_var("XDG_CONFIG_HOME", tmp_home.join(".config"));
    env::set_var("XDG_DATA_HOME", tmp_home.join(".local/share"));

    dir
}

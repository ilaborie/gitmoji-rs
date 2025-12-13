#![allow(clippy::missing_panics_doc)]

use std::{env, fs};

use assert_fs::TempDir;

#[must_use]
pub fn home_isolation() -> TempDir {
    let dir = TempDir::new().expect("Create temp dir for HOME isolation");
    fs::create_dir_all(dir.path()).expect("Create HOME directory");
    let tmp_home = dir.path().to_str().expect("Convert path to string");
    env::set_var("HOME", tmp_home);

    dir
}

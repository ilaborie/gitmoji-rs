use std::{env, fs};

use assert_fs::TempDir;

pub fn home_isolation<'a>() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path()).unwrap();
    let tmp_home = dir.path().to_str().unwrap();
    env::set_var("HOME", tmp_home);

    dir
}

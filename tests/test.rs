use assert2::check;
use directories::BaseDirs;
use serial_test::serial;

mod common;
pub use self::common::*;

#[test_log::test]
#[serial]
fn test_home_isolation() {
    let temp_dir = home_isolation();
    let base_dirs = BaseDirs::new().unwrap();

    check!(base_dirs.home_dir().to_path_buf() == temp_dir.path());
}

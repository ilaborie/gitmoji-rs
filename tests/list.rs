use gitmoji_rs::EXIT_NO_CONFIG;
use serial_test::serial;

mod common;
pub use self::common::*;

#[test_log::test(tokio::test)]
#[serial]
async fn should_have_list_command() {
    let _dir = home_isolation();
    setup_test_config().await;

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("list");
    cmd.assert().success();
}

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "Fail on CI"]
async fn should_have_list_command_fail_without_config() {
    let _dir = home_isolation();

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("list");
    cmd.assert().failure();
    cmd.assert().code(EXIT_NO_CONFIG);
}

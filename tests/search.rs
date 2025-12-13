use gitmoji_rs::EXIT_NO_CONFIG;
use serial_test::serial;

mod common;
pub use self::common::*;

#[test_log::test(tokio::test)]
#[serial]
async fn should_have_search_command() {
    let _dir = home_isolation();
    setup_test_config().await;

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.args(["search", "t"]);
    cmd.assert().success();
}

#[test_log::test(tokio::test)]
#[serial]
async fn should_have_search_command_missing_arg() {
    let _dir = home_isolation();
    setup_test_config().await;

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("search");
    cmd.assert().failure();
}

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "Fail on CI"]
async fn should_have_search_command_fail_without_config() {
    let _dir = home_isolation();

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("search");
    cmd.arg("test");
    cmd.assert().failure();
    cmd.assert().code(EXIT_NO_CONFIG);
}

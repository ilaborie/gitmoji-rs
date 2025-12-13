use gitmoji_rs::{write_config, Gitmoji, GitmojiConfig, EXIT_NO_CONFIG};
use serial_test::serial;

mod common;
pub use self::common::*;

#[test_log::test(tokio::test)]
#[serial]
async fn should_have_list_command() {
    let _dir = home_isolation();
    let mut config = GitmojiConfig::default();
    config.set_gitmojis(vec![Gitmoji::new(
        String::from("🧪"),
        String::from(":test_tube:"),
        Some(String::from("A Name")),
        Some(String::from("A description")),
    )]);
    write_config(&config).await.unwrap();

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("list");

    let _ = dbg!(cmd.ok());
    cmd.assert().success();
}

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "Fail on CI"]
async fn should_have_list_command_fail_without_config() {
    let _dir = home_isolation();

    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("list");

    let _ = dbg!(cmd.ok());
    cmd.assert().failure();
    cmd.assert().code(EXIT_NO_CONFIG);
}

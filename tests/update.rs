use assert_cmd::Command;
use gitmoji_rs::{write_config, EmojiFormat, GitmojiConfig, EXIT_CANNOT_UPDATE};
use serial_test::serial;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
pub use self::common::*;

#[test_log::test(tokio::test)]
#[serial]
async fn should_have_update_command() {
    let _dir = home_isolation();

    let mock_server = MockServer::start().await;
    let url = format!("{}/gitmoji", &mock_server.uri());
    let config = GitmojiConfig::new(
        false,
        EmojiFormat::UseCode,
        false,
        false,
        url.parse().unwrap(),
    );
    write_config(&config).await.unwrap();

    Mock::given(method("GET"))
        .and(path("/gitmoji"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{
    "gitmojis": [
        {
            "emoji": "🎨",
            "entity": "&#x1f3a8;",
            "code": ":art:",
            "description": "Improve structure / format of the code.",
            "name": "art",
            "semver": null
        }
    ]
}"#,
            "application/json",
        ))
        .mount(&mock_server)
        .await;

    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.arg("update");

    let _ = dbg!(cmd.ok());
    cmd.assert().success();
}

#[test_log::test(tokio::test)]
#[serial]
async fn should_have_update_command_fail_bad_config() {
    let _dir = home_isolation();

    let mock_server = MockServer::start().await;
    let url = format!("{}/gitmoji", &mock_server.uri());
    let config = GitmojiConfig::new(
        false,
        EmojiFormat::UseCode,
        false,
        false,
        url.parse().unwrap(),
    );
    write_config(&config).await.unwrap();

    Mock::given(method("GET"))
        .and(path("/gitmoji"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.arg("update");

    let _ = dbg!(cmd.ok());
    cmd.assert().code(EXIT_CANNOT_UPDATE);
}

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "does not work on CI"]
async fn should_have_update_command_without_config() {
    let _dir = home_isolation();

    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.arg("update");

    let _ = dbg!(cmd.ok());
    cmd.assert().success();
}

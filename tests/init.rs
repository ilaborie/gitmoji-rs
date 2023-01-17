use assert2::check;
use assert_cmd::Command;
use gitmoji_rs::{read_config_or_fail, EmojiFormat, GitmojiConfig, DEFAULT_URL};
use rexpect::session::spawn_command;
use serial_test::serial;

mod common;
pub use self::common::*;

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "does not work on CI"]
async fn should_have_init_command_default_values() -> Result<(), rexpect::error::Error> {
    let _dir = home_isolation();

    let mut cmd = cargo_bin_command("gitmoji");
    cmd.arg("-v");
    cmd.arg("init");

    let mut p = spawn_command(cmd, Some(10_000))?;

    // Auto git add
    p.exp_regex("Enable automatic \"git add .\"")?;
    p.send_line("")?;

    // Format
    p.exp_regex("Select how emojis should be used in commits")?;
    p.send_line("")?;

    // Signed
    p.exp_regex("Enable signed commits")?;
    p.send_line("")?;

    // Scope
    p.exp_regex("Enable scope prompt")?;
    p.send_line("")?;

    // URL
    p.exp_regex("Set gitmojis api url")?;
    p.send_line("")?;

    p.exp_eof()?;

    // Check config exists
    let config = read_config_or_fail().await.unwrap();
    check!(config.auto_add() == false);
    check!(*config.format() == EmojiFormat::UseCode);
    check!(config.signed() == false);
    check!(config.scope() == false);
    check!(config.update_url() == DEFAULT_URL);

    Ok(())
}

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "depends on working directory"]
async fn should_have_init_command_default_flag() -> anyhow::Result<()> {
    let _dir = home_isolation();
    let mut cmd = Command::cargo_bin("gitmoji")?;
    cmd.arg("init");
    cmd.arg("--default");

    cmd.assert().success();

    // Check config exists
    let config = read_config_or_fail().await?;
    check!(config.auto_add() == false);
    check!(*config.format() == EmojiFormat::UseCode);
    check!(config.signed() == false);
    check!(config.scope() == false);
    check!(config.update_url() == DEFAULT_URL);

    Ok(())
}

async fn gitmoji_init(
    git_add: bool,
    format: EmojiFormat,
    signed: bool,
    scope: bool,
    url: &str,
) -> Result<GitmojiConfig, rexpect::error::Error> {
    let _dir = home_isolation();

    let mut cmd = cargo_bin_command("gitmoji");
    cmd.arg("-v");
    cmd.arg("init");

    let mut p = spawn_command(cmd, Some(10_000))?;

    // Auto git add
    p.exp_regex("Enable automatic \"git add .\"")?;
    let answer = if git_add { "y" } else { "n" };
    p.send_line(answer)?;

    // Format
    p.exp_regex("Select how emojis should be used in commits")?;
    let answer = match format {
        EmojiFormat::UseCode => "",
        EmojiFormat::UseEmoji => "\x1b[B", // "ESC[1B";
    };
    p.send_line(answer)?;

    // Signed
    p.exp_regex("Enable signed commits")?;
    let answer = if signed { "y" } else { "n" };
    p.send_line(answer)?;

    // Scope
    p.exp_regex("Enable scope prompt")?;
    let answer = if scope { "y" } else { "n" };
    p.send_line(answer)?;

    // URL
    p.exp_regex("Set gitmojis api url")?;
    p.send_line(url)?;

    p.exp_eof()?;

    let config = read_config_or_fail().await.unwrap();
    Ok(config)
}

#[test_log::test(tokio::test)]
#[ignore = "does not work"]
async fn should_have_init_command_use_values() -> anyhow::Result<()> {
    let url = "http://plop.org";

    let config = gitmoji_init(true, EmojiFormat::UseEmoji, true, true, url)
        .await
        .unwrap();

    // Check config exists
    check!(config.auto_add() == true);
    check!(*config.format() == EmojiFormat::UseEmoji);
    check!(config.signed() == true);
    check!(config.scope() == true);
    check!(config.update_url() == url);

    Ok(())
}

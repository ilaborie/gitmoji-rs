use assert_cmd::Command;
use rstest::rstest;

mod common;
pub use self::common::*;

#[test_log::test]
fn should_have_help_command() {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.arg("help");

    cmd.assert().success();
}

#[test_log::test]
fn should_have_help_flag_short() {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.arg("-h");

    cmd.assert().success();
}

#[test_log::test]
fn should_have_help_flag_long() {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.arg("--help");

    cmd.assert().success();
}

#[rstest]
#[case::help("help")]
#[case::init("init")]
#[case::search("commit")]
#[case::search("update")]
#[case::search("search")]
#[case::list("list")]
#[test_log::test]
fn should_have_help_for_subcommand_command(#[case] subcommand: &str) {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.args(["help", subcommand]);

    cmd.assert().success();
}

#[rstest]
#[case::init("init")]
#[case::search("commit")]
#[case::search("update")]
#[case::search("search")]
#[case::list("list")]
#[test_log::test]
fn should_have_help_for_subcommand_command_with_short_flag(#[case] subcommand: &str) {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.args([subcommand, "-h"]);

    cmd.assert().success();
}

#[rstest]
#[case::init("init")]
#[case::search("commit")]
#[case::search("update")]
#[case::search("search")]
#[case::list("list")]
#[test_log::test]
fn should_have_help_for_subcommand_command_with_long_flag(#[case] subcommand: &str) {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.args([subcommand, "--help"]);

    cmd.assert().success();
}

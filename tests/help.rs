use rstest::rstest;

mod common;
pub use self::common::*;

#[test_log::test]
fn should_have_help_command() {
    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("help");

    cmd.assert().success();
}

#[test_log::test]
fn should_have_help_flag_short() {
    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.arg("-h");

    cmd.assert().success();
}

#[test_log::test]
fn should_have_help_flag_long() {
    let mut cmd = assert_cargo_bin("gitmoji");
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
    let mut cmd = assert_cargo_bin("gitmoji");
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
    let mut cmd = assert_cargo_bin("gitmoji");
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
    let mut cmd = assert_cargo_bin("gitmoji");
    cmd.args([subcommand, "--help"]);

    cmd.assert().success();
}

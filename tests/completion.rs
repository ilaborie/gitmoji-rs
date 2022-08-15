use assert_cmd::Command;
use rstest::rstest;

mod common;
pub use self::common::*;

#[rstest]
#[case::bash("bash")]
#[case::zsh("zsh")]
#[case::elvish("elvish")]
#[case::fish("fish")]
#[case::powershell("powershell")]
#[test_log::test]
fn should_have_completion_command(#[case] shell: &str) {
    let mut cmd = Command::cargo_bin("gitmoji").unwrap();
    cmd.args(["completion", shell]);

    cmd.assert().success();
}

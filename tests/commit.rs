use rexpect::session::spawn_command;
use serial_test::serial;

mod common;
pub use self::common::*;

#[test_log::test(tokio::test)]
#[serial]
#[ignore = "does not work"]
async fn should_have_commit_command() -> Result<(), rexpect::error::Error> {
    let _dir = home_isolation();
    let git_repo = GitRepository::default();
    git_repo.touch("plop.txt");

    setup_test_config().await;

    let mut cmd = cargo_bin_command("gitmoji");
    cmd.current_dir(git_repo.path());
    cmd.arg("commit");

    let mut p = spawn_command(cmd, Some(10_000))?;
    p.send_line("")?;
    p.send_line("title")?;
    p.send_line("body")?;

    p.exp_eof()?;

    let _list = git_repo.list_commits(None);

    Ok(())
}

use gitmoji_rs::{write_config, Gitmoji, GitmojiConfig};
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

    let mut config = GitmojiConfig::default();
    config.set_gitmojis(vec![Gitmoji::new(
        String::from("🧪"),
        String::from(":test_tube:"),
        Some(String::from("A Name")),
        Some(String::from("A description")),
    )]);
    write_config(&config).await.unwrap();

    let mut cmd = cargo_bin_command("gitmoji");
    cmd.current_dir(dbg!(git_repo.path()));
    cmd.arg("commit");

    let mut p = spawn_command(cmd, Some(10_000))?;
    p.send_line("")?;
    p.send_line("title")?;
    p.send_line("body")?;

    p.exp_eof()?;

    let list = git_repo.list_commits(None);
    dbg!(&list);

    Ok(())
}

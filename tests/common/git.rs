use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use assert_fs::fixture::{FileTouch, PathChild};
use assert_fs::TempDir;

pub struct GitRepository {
    #[allow(dead_code)]
    dir: TempDir,
    root: PathBuf,
}

impl GitRepository {
    pub fn new() -> Self {
        let dir = TempDir::new().unwrap();
        let root = dir.path().to_path_buf();

        let result = Self { dir, root };
        result.init();

        result
    }

    pub fn path(&self) -> PathBuf {
        self.root.to_path_buf()
    }

    fn init(&self) {
        let status = Command::new("git")
            .current_dir(&self.root)
            .args(["init"])
            .status()
            .unwrap();

        if !status.success() {
            panic!(
                "Fail to create a git repository in {:?}, status: {status:?}",
                self.root
            )
        }
    }

    pub fn touch(&self, file: &str) {
        let child = self.dir.child(file);
        child.touch().expect("Expect to touch file");
    }

    pub fn stage(&self, file: &str) {
        let status = Command::new("git")
            .current_dir(&self.root)
            .args(["add", file])
            .status()
            .unwrap();

        if !status.success() {
            panic!("Fail to add {file} to index, status: {status:?}")
        }
    }

    pub fn list_commits(&self, commit_ref: Option<String>) -> Vec<GitCommit> {
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.root);
        cmd.args([
            "log",
            "--oneline",
            "--no-color",
            "--no-decorate",
            "--no-abbrev-commit",
        ]);
        if let Some(commit_ref) = commit_ref {
            cmd.arg(format!("{commit_ref}.."));
        }

        let output = cmd.output().unwrap();
        let status = output.status;
        if !status.success() {
            panic!("Fail to list commits, status: {status:?} with {output:?}");
        }

        let mut result = vec![];
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let commit = line.parse().unwrap();
            result.push(commit);
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct GitCommit {
    id: String,
    message: String,
}

impl GitCommit {
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

impl FromStr for GitCommit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let id = split
            .next()
            .ok_or_else(|| anyhow::Error::msg("Commit id not found"))?
            .to_string();
        let message = split
            .next()
            .ok_or_else(|| anyhow::Error::msg("Commit id not found"))?
            .to_string();
        Ok(Self { id, message })
    }
}

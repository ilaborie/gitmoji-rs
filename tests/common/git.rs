#![allow(clippy::missing_panics_doc)]

use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use anyhow::Ok;
use assert_fs::fixture::{FileTouch, PathChild};
use assert_fs::TempDir;

pub struct GitRepository {
    dir: TempDir,
    root: PathBuf,
}

impl Default for GitRepository {
    fn default() -> Self {
        let dir = TempDir::new().expect("Create temp dir");
        let root = dir.path().to_path_buf();

        let result = Self { dir, root };
        result.init();

        result
    }
}

impl GitRepository {
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.root.clone()
    }

    fn init(&self) {
        let status = Command::new("git")
            .current_dir(&self.root)
            .args(["init"])
            .status()
            .expect("Run git init");

        assert!(
            status.success(),
            "Fail to create a git repository in {}, status: {status:?}",
            self.root.display()
        );
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
            .expect("Run git add");

        assert!(
            status.success(),
            "Fail to add {file} to index, status: {status:?}"
        );
    }

    #[must_use]
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

        let output = cmd.output().expect("Run git log");
        let status = output.status;
        assert!(
            status.success(),
            "Fail to list commits, status: {status:?} with {output:?}"
        );

        let mut result = vec![];
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let commit = line.parse().expect("Parse commit");
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
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    #[must_use]
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

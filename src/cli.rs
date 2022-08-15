use clap::{Parser, Subcommand};
use clap_complete::Shell;
use url::Url;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
/// A gitmoji client for using emojis on commit messages.
pub struct Settings {
    #[clap(subcommand)]
    pub(crate) command: Command,

    #[clap(short, long)]
    /// Verbose mode
    verbose: bool,
}

impl Settings {
    /// Is verbose mode toggled
    #[must_use]
    pub const fn verbose(&self) -> bool {
        self.verbose
    }

    /// Get the command
    #[must_use]
    pub const fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Debug, Clone, Subcommand)]
/// Available commands
pub enum Command {
    /// Setup gitmoji preferences
    Init {
        // TODO [#3] allow local
        #[clap(long)]
        /// Use default configuration without interactivity
        default: bool,
    },

    /// Interactively commit using the prompts
    Commit {
        #[clap(long)]
        /// Add the `--all` flag for git commit command
        all: bool,

        #[clap(long)]
        /// Add the `--amend` flag for git commit command
        amend: bool,
    },

    /// Sync emoji list with the repository
    Update {
        /// Change the update URL
        url: Option<Url>,
    },

    /// List all available gitmojis
    List,

    /// Search gitmojis
    Search {
        /// Search text
        text: String,
    },

    /// Create or remove git commit hook
    #[cfg(feature = "hook")]
    #[clap(subcommand)]
    Hook(HookOperation),

    /// Generate completion for shell
    Completion {
        /// The shell (bash, zsh, fish, elvish, powershell)
        shell: Shell,
    },
}

#[cfg(feature = "hook")]
#[derive(Debug, Clone, Subcommand)]
/// Available hook operation
pub enum HookOperation {
    /// Add the hook
    Add,
    /// Remove the hook
    Remove,
    /// (Used by the hook to create commit message)
    Apply {
        /// The commit message file
        dest: std::path::PathBuf,

        /// The commit source
        source: Option<String>,
    },
}

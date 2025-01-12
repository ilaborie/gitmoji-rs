use crate::git::GitCommandError;

#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
/// Gitmojis errors
#[non_exhaustive]
pub enum Error {
    /// Cannot retrieve gitmojis
    #[display("Cannot retrieve gitmojis: {_0:?}")]
    CannotFetchGitmojis(reqwest::Error),

    /// Cannot get the project config file
    #[display("Cannot get project config because {cause}")]
    CannotGetProjectConfigFile {
        /// The cause
        cause: String,
    },

    /// Cannot commit
    #[display("Fail to commit")]
    FailToCommit,

    /// Configuration file not found
    #[display("Missing the configuration file, to create it use `gitmoji config`")]
    MissingConfigFile,

    /// I/O error
    IoError(std::io::Error),

    /// Issue while running a git command
    GitCommandError(GitCommandError),

    /// Invalid URL
    InvalidUrlError(url::ParseError),

    /// TOML serialization error
    TomlSerializeError(toml_edit::ser::Error),

    /// TOML deserialization error
    TomlDeserializeError(toml_edit::de::Error),

    /// A Dialoguer error
    DialoguerError(dialoguer::Error),
}

/// Gitmojis result
pub type Result<T> = std::result::Result<T, Error>;

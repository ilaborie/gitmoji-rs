#[derive(Debug, thiserror::Error)]
/// Gitmojis errors
pub enum Error {
    #[error(transparent)]
    /// Cannot retrieve gitmojis
    CannotFetchGitmojis(#[from] reqwest::Error),

    #[error("Cannot get project config because {0}")]
    /// Cannot get the project config file
    CannotGetProjectConfigFile(String),

    #[error("Fail to commit")]
    /// Cannot commit
    FailToCommit,

    #[error("Missing the configuration file, to create it use `gitmoji config`")]
    /// Configuration file not found
    MissingConfigFile,

    #[error(transparent)]
    /// I/O error
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    /// Invalid URL
    InvalidUrlError(#[from] url::ParseError),

    #[error(transparent)]
    /// TOML serialization error
    TomlSerializeError(#[from] toml::ser::Error),

    #[error(transparent)]
    /// TOML deserialization error
    TomlDeserializeError(#[from] toml::de::Error),
}

/// Gitmojis result
pub type Result<T> = std::result::Result<T, Error>;

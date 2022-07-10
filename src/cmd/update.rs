use serde::Deserialize;

use super::write_config;
use crate::model::GitmojiConfig;
use crate::{Gitmoji, Result};

#[derive(Debug, Clone, Default, Deserialize)]
struct GetGitmojis {
    gitmojis: Vec<Gitmoji>,
}

pub async fn update_gitmojis(mut config: GitmojiConfig) -> Result<GitmojiConfig> {
    let result = reqwest::get(config.update_url())
        .await?
        .json::<GetGitmojis>()
        .await?;
    config.set_gitmojis(result.gitmojis);
    write_config(&config).await?;

    Ok(config)
}

use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};

use crate::{Gitmoji, GitmojiConfig, Result};

pub struct CommitParams {
    pub gitmoji: Gitmoji,
    pub scope: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

pub fn get_commit_params(config: &GitmojiConfig, term: &Term) -> Result<CommitParams> {
    let theme = ColorfulTheme::default();

    let gitmoji_idx = FuzzySelect::with_theme(&theme)
        .with_prompt("Pick your flavor")
        .items(config.gitmojis())
        .default(0)
        .interact_on(term)?;
    let gitmoji = config
        .gitmojis()
        .get(gitmoji_idx)
        .expect("Should be in bounds")
        .clone();
    let scope = if config.scope() {
        // TODO: [#2] add an history
        let scope = Input::with_theme(&theme)
            .with_prompt("Enter the scope of current changes:")
            .default("*".to_string())
            .interact_text_on(term)?;
        Some(scope)
    } else {
        None
    };
    let title = Input::with_theme(&theme)
        .with_prompt("Enter the commit title")
        .allow_empty(false)
        .interact_text_on(term)?;
    let description: String = Input::with_theme(&theme)
        .with_prompt("Enter the commit message:")
        .allow_empty(true)
        .interact_text_on(term)?;
    let description = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    let result = CommitParams {
        gitmoji,
        scope,
        title,
        description,
    };
    Ok(result)
}

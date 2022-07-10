use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};

use crate::{Gitmoji, GitmojiConfig, Result};

pub struct CommitParams {
    pub gitmoji: Gitmoji,
    pub scope: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

pub fn get_commit_params(config: &GitmojiConfig) -> Result<CommitParams> {
    let theme = ColorfulTheme::default();

    let gitmoji_idx = FuzzySelect::with_theme(&theme)
        .with_prompt("Pick your flavor")
        .items(config.gitmojis())
        .interact()?;
    let gitmoji = config
        .gitmojis()
        .iter()
        .nth(gitmoji_idx)
        .expect("Should be in bounds")
        .clone();
    let scope = if config.scope() {
        // TODO: [#2] add an history
        let scope = Input::with_theme(&theme)
            .with_prompt("Enter the scope of current changes:")
            .default("*".to_string())
            .interact_text()?;
        Some(scope)
    } else {
        None
    };
    let title = Input::with_theme(&theme)
        .with_prompt("Enter the commit title")
        .allow_empty(false)
        .interact_text()?;
    let description: String = Input::with_theme(&theme)
        .with_prompt("Enter the commit message:")
        .allow_empty(true)
        .interact_text()?;
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

//     let theme = ColorfulTheme::default();
//     let auto_add = Confirm::with_theme(&theme)
//         .with_prompt(r#"Enable automatic "git add .""#)
//         .default(false)
//         .interact()?;
//     let format_idx = Select::with_theme(&theme)
//         .with_prompt("Select how emojis should be used in commits")
//         .default(0)
//         .items(FORMAT_ITEMS)
//         .interact()?;
//     let format = FORMAT_ITEMS[format_idx].value;
//     let signed = Confirm::with_theme(&theme)
//         .with_prompt("Enable signed commits")
//         .default(false)
//         .interact()?;
//     let scope = Confirm::with_theme(&theme)
//         .with_prompt("Enable scope prompt")
//         .default(false)
//         .interact()?;
//     let update_url = Input::with_theme(&theme)
//         .with_prompt("Set gitmojis api url")
//         .default(DEFAULT_URL.to_string())
//         .validate_with(validate_url)
//         .interact_text()?
//         .parse()?;

//     let config = GitmojiConfig::new(auto_add, format, signed, scope, update_url);
//     Ok(config)
// }

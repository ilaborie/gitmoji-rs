use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::Gitmoji;

const WEIGHT_NAME: i64 = 65;
const WEIGHT_DESCRIPTION: i64 = 35;
const MAX_LENGTH: usize = 5;

pub(super) fn filter(gitmojis: &[Gitmoji], text: &str) -> Vec<Gitmoji> {
    let matcher = SkimMatcherV2::default();

    let mut filtered = gitmojis
        .iter()
        .filter_map(|gitmoji| {
            let name_score = matcher.fuzzy_match(gitmoji.name().unwrap_or_default(), text);
            let desc_score = matcher.fuzzy_match(gitmoji.description().unwrap_or_default(), text);

            let score = match (name_score, desc_score) {
                (Some(ns), Some(ds)) => Some(WEIGHT_NAME * ns + WEIGHT_DESCRIPTION * ds),
                (Some(ns), None) => Some(WEIGHT_NAME * ns),
                (None, Some(ds)) => Some(WEIGHT_DESCRIPTION * ds),
                (None, None) => None,
            };

            score.map(|score| (gitmoji.clone(), score))
        })
        .collect::<Vec<_>>();

    filtered.sort_by(|a, b| b.1.cmp(&a.1));
    filtered.truncate(MAX_LENGTH);

    filtered.into_iter().map(|(gitmoji, _)| gitmoji).collect()
}

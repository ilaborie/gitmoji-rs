use console::Style;

use crate::Gitmoji;

pub(super) fn print_gitmojis(gitmojis: &[Gitmoji]) {
    let blue = Style::new().blue();
    for gitmoji in gitmojis {
        let emoji = gitmoji.emoji();
        let code = gitmoji.code();
        let description = gitmoji.description().unwrap_or_default();
        println!("{emoji}\t{}\t{description}", blue.apply_to(code));
    }
}

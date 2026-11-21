use super::*;
use crate::theme::color::{contrast_ratio, AA_TEXT_CONTRAST};
use crate::theme::derive::derive_default_tokens;
use crate::theme::{PaletteMode, ThemeRegistry};

#[test]
fn an_untinted_card_keeps_the_feed_surface() {
    let theme = ThemeTokens::default();
    let plain = card_surface(None, false, &theme);
    assert_eq!((plain.background, plain.hover, plain.rule, plain.edge), (theme.bg_feed_post, theme.hover_bg, theme.border_subtle, None));
    let focused = card_surface(None, true, &theme);
    assert_eq!((focused.background, focused.hover), (theme.bg_elevated, theme.bg_elevated));
    assert_eq!(row_tint(None, &theme), None);
}

#[test]
fn a_tinted_card_takes_the_tint_fill_and_border() {
    let theme = ThemeTokens::default();
    for &tint in Tint::ALL {
        let colors = tint_colors(tint, &theme);
        let card = card_surface(Some(tint), false, &theme);
        assert_eq!((card.background, card.rule, card.edge), (colors.background, colors.border, Some(colors.border)), "{}", tint.key());
        assert_ne!(card.hover, card.background, "{}: hover still shows", tint.key());
        let focused = card_surface(Some(tint), true, &theme);
        assert_eq!((focused.background, focused.hover), (colors.background, colors.background), "{}: focus keeps the tint", tint.key());
        assert_eq!(row_tint(Some(tint), &theme), Some((colors.background, colors.border)));
    }
}

#[test]
fn text_on_a_tinted_card_is_readable_in_both_themes() {
    let mut modes = Vec::new();
    for palette in ThemeRegistry::global().all() {
        let t = derive_default_tokens(palette);
        modes.push(t.mode);
        for &tint in Tint::ALL {
            let card = card_surface(Some(tint), false, &t);
            for (role, text) in [("text_primary", t.text_primary), ("text_secondary", t.text_secondary), ("text_muted", t.text_muted)] {
                let ratio = contrast_ratio(text, card.background);
                assert!(ratio >= AA_TEXT_CONTRAST, "{}: {role} on {} = {ratio:.2}", palette.id, tint.key());
            }
        }
    }
    assert!(modes.contains(&PaletteMode::Light) && modes.contains(&PaletteMode::Dark), "both modes are covered");
}

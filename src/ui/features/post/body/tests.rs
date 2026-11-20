//! Body layout: words, mark ranges, breaks and code items; mark highlights from theme tokens.

use super::layout::{blank_line_after, layout, Item, MarkSet};
use super::marks::mark_highlight;
use crate::domain::post::tokenize_post_body;
use crate::theme::ThemeTokens;
use gpui_kit::{FontStyle, FontWeight};

const BOLD: MarkSet = MarkSet { bold: true, italic: false, underline: false, spoiler: false };
const BOLD_ITALIC: MarkSet = MarkSet { bold: true, italic: true, underline: false, spoiler: false };

fn word(text: &str) -> Item<'static> {
    Item::Word { text: text.to_string(), marks: Vec::new() }
}

fn words_of(source: &str) -> Vec<String> {
    let tokens = tokenize_post_body(source);
    layout(&tokens)
        .into_iter()
        .filter_map(|item| match item {
            Item::Word { text, .. } => Some(text),
            _ => None,
        })
        .collect()
}

#[test]
fn text_wraps_word_by_word() {
    assert_eq!(words_of("  Substance is one\tand  infinite; "), vec!["Substance", "is", "one", "and", "infinite;"]);
    assert!(words_of("   ").is_empty());
    assert_eq!(words_of("実体は一つで無限である").len(), 1);
}

#[test]
fn a_word_keeps_the_byte_ranges_of_its_marks() {
    let tokens = tokenize_post_body("un**believ**able ***wow***");
    let items = layout(&tokens);
    assert_eq!(
        items,
        vec![
            Item::Word { text: "unbelievable".to_string(), marks: vec![(2..8, BOLD)] },
            Item::Word { text: "wow".to_string(), marks: vec![(0..3, BOLD_ITALIC)] },
        ]
    );
}

#[test]
fn marked_runs_split_into_words_with_their_marks() {
    let tokens = tokenize_post_body("**two words**");
    assert_eq!(
        layout(&tokens),
        vec![
            Item::Word { text: "two".to_string(), marks: vec![(0..3, BOLD)] },
            Item::Word { text: "words".to_string(), marks: vec![(0..5, BOLD)] },
        ]
    );
}

#[test]
fn breaks_code_and_blocks_are_their_own_items() {
    let tokens = tokenize_post_body("a\n\nrun `x`\n```py\nprint()\n```");
    let items = layout(&tokens);
    assert_eq!(
        items,
        vec![
            word("a"),
            Item::Break,
            Item::Break,
            word("run"),
            Item::Code("x"),
            Item::Block { lang: Some("py"), code: "print()" },
        ]
    );
    assert!(blank_line_after(&items, 1), "the line after the first break is blank");
    assert!(!blank_line_after(&items, 2), "the line after the second break has words");
}

#[test]
fn links_and_tags_inside_marks_stay_atoms() {
    let tokens = tokenize_post_body("**see [[p1]] #tag!**");
    let items = layout(&tokens);
    assert_eq!(
        items,
        vec![
            Item::Word { text: "see".to_string(), marks: vec![(0..3, BOLD)] },
            Item::Wikilink { target: "p1", trailing: "" },
            Item::Hashtag { tag: "#tag", trailing: "!" },
        ]
    );
}

#[test]
fn highlights_come_from_marks_and_theme_tokens() {
    let theme = ThemeTokens::default();
    assert_eq!(mark_highlight(MarkSet::default(), &theme), None);
    let spoiler_only = MarkSet { spoiler: true, ..MarkSet::default() };
    assert_eq!(mark_highlight(spoiler_only, &theme), None, "spoilers show plainly for now");
    let style = mark_highlight(BOLD_ITALIC, &theme).expect("bold italic draws");
    assert_eq!(style.font_weight, Some(FontWeight::BOLD));
    assert_eq!(style.font_style, Some(FontStyle::Italic));
    assert!(style.underline.is_none() && style.color.is_none() && style.background_color.is_none());
    let underline = mark_highlight(MarkSet { underline: true, ..MarkSet::default() }, &theme).expect("underline draws");
    assert_eq!(underline.underline.map(|u| u.thickness), Some(theme.border_width_hairline));
}

//! Body layout: words, mark ranges, breaks and code items; mark highlights from theme tokens.

use super::code_block::{code_block, language_label, style_color};
use super::layout::{blank_line_after, layout, Item, MarkSet};
use super::marks::mark_highlight;
use super::spoiler::{hidden_spoiler, is_revealed, reveal, revealed_spoiler, SpoilerKey};
use crate::domain::post::highlight::CodeStyle;
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
fn code_block_colors_are_theme_tokens_and_the_label_names_the_language() {
    let theme = ThemeTokens::default();
    assert_eq!(style_color(CodeStyle::Keyword, &theme), theme.code_keyword);
    assert_eq!(style_color(CodeStyle::Type, &theme), theme.code_type);
    assert_eq!(style_color(CodeStyle::String, &theme), theme.code_string);
    assert_eq!(style_color(CodeStyle::Number, &theme), theme.code_number);
    assert_eq!(style_color(CodeStyle::Comment, &theme), theme.code_comment);
    assert_eq!(language_label(Some("rust")), "rust");
    let plain = language_label(None);
    assert!(!plain.is_empty() && plain != "post.code_plain", "a block without a tag is labelled as plain text");
}

#[test]
fn a_code_block_renders_highlighted_and_plain() {
    let theme = ThemeTokens::default();
    let _rust = code_block(0, Some("rust"), "fn main() {\n\n    let x = 1; // one\n}", &theme, Default::default());
    let _plain = code_block(1, None, "", &theme, Default::default());
}

#[test]
fn a_spoiler_is_one_group_holding_its_own_items() {
    let tokens = tokenize_post_body("a ||b **c**|| d");
    let inside = MarkSet { spoiler: true, ..MarkSet::default() };
    let bold_inside = MarkSet { bold: true, ..inside };
    assert_eq!(
        layout(&tokens),
        vec![
            word("a"),
            Item::Spoiler {
                ordinal: 0,
                text: "b c".to_string(),
                items: vec![
                    Item::Word { text: "b".to_string(), marks: vec![(0..1, inside)] },
                    Item::Word { text: "c".to_string(), marks: vec![(0..1, bold_inside)] },
                ],
            },
            word("d"),
        ]
    );
}

#[test]
fn spoilers_are_numbered_in_reading_order() {
    let tokens = tokenize_post_body("||one|| then ||two [[p1]]||");
    let spoilers: Vec<(usize, String)> = layout(&tokens)
        .into_iter()
        .filter_map(|item| match item {
            Item::Spoiler { ordinal, text, .. } => Some((ordinal, text)),
            _ => None,
        })
        .collect();
    assert_eq!(spoilers, vec![(0, "one".to_string()), (1, "two p1".to_string())]);
}

#[test]
fn a_reveal_lasts_for_the_session_and_stays_with_its_post() {
    let key = |scope: &str| SpoilerKey { scope: scope.to_string(), ordinal: 0, text: "ending".to_string() };
    assert!(!is_revealed(&key("p1")));
    reveal(key("p1"));
    assert!(is_revealed(&key("p1")), "revealed until the app closes");
    assert!(!is_revealed(&key("p2")), "the same text in another post stays hidden");
    assert!(!is_revealed(&SpoilerKey { text: "edited".to_string(), ..key("p1") }), "edited spoiler text hides again");
}

#[test]
fn hidden_and_revealed_spoilers_render() {
    let theme = ThemeTokens::default();
    let _hidden = hidden_spoiler(0, SpoilerKey { scope: String::new(), ordinal: 0, text: "x".to_string() }, &theme);
    let _revealed = revealed_spoiler(Vec::new(), &theme);
    assert_ne!(theme.spoiler_fill, theme.bg_feed_post);
}

#[test]
fn highlights_come_from_marks_and_theme_tokens() {
    let theme = ThemeTokens::default();
    assert_eq!(mark_highlight(MarkSet::default(), &theme), None);
    let spoiler_only = MarkSet { spoiler: true, ..MarkSet::default() };
    assert_eq!(mark_highlight(spoiler_only, &theme), None, "the spoiler group hides its text, not a highlight");
    let style = mark_highlight(BOLD_ITALIC, &theme).expect("bold italic draws");
    assert_eq!(style.font_weight, Some(FontWeight::BOLD));
    assert_eq!(style.font_style, Some(FontStyle::Italic));
    assert!(style.underline.is_none() && style.color.is_none() && style.background_color.is_none());
    let underline = mark_highlight(MarkSet { underline: true, ..MarkSet::default() }, &theme).expect("underline draws");
    assert_eq!(underline.underline.map(|u| u.thickness), Some(theme.border_width_hairline));
}

mod links;
mod code_run; // P5

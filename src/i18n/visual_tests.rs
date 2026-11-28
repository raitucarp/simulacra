//! T21: Arabic in visual order for the left-to-right renderer.

use super::visual_order;
use std::borrow::Cow;

fn chars(codes: &[u32]) -> String {
    codes.iter().filter_map(|code| char::from_u32(*code)).collect()
}

#[test]
fn text_without_arabic_is_left_alone() {
    assert!(matches!(visual_order("Create network (3)"), Cow::Borrowed("Create network (3)")));
}

#[test]
fn a_word_is_joined_and_drawn_from_the_right() {
    // ا ل ع ر ب ي ة: isolated alef, initial lam, medial ain, final reh, initial beh, medial yeh,
    // final teh marbuta — then reversed for the left-to-right renderer.
    let visual = chars(&[0xFE94, 0xFEF4, 0xFE91, 0xFEAE, 0xFECC, 0xFEDF, 0xFE8D]);
    assert_eq!(visual_order("العربية"), visual);
}

#[test]
fn lam_and_alef_become_one_ligature() {
    assert_eq!(visual_order("لا"), chars(&[0xFEFB]));
}

#[test]
fn latin_words_keep_their_order_inside_arabic() {
    let visual = visual_order("مرحبا Simulacra");
    assert!(visual.starts_with("Simulacra "), "{visual}");
}

#[test]
fn placeholders_survive_whole() {
    let visual = visual_order("أهلاً %{name}، مرحبا");
    assert!(visual.contains("%{name}"), "{visual}");
}

#[test]
fn brackets_still_open_and_close_the_right_way() {
    let visual = visual_order("(مرحبا)");
    assert!(visual.starts_with('(') && visual.ends_with(')'), "{visual}");
}

#[test]
fn a_mark_stays_on_its_letter() {
    let visual: Vec<char> = visual_order("بّ").chars().collect();
    assert_eq!(visual, vec!['\u{FE8F}', '\u{0651}']);
}

#[test]
fn the_language_menu_names_arabic_readably() {
    assert_eq!(crate::i18n::Language::Arabic.display_label(), visual_order("العربية"));
    assert_eq!(crate::i18n::Language::French.display_label(), "Français");
}

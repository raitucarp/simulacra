//! Inserting syntax tokens (`#`, `[[`) at the composer cursor as the start of a new word.

use gpui_kit::component::input::TextareaState;
use gpui_kit::{App, Entity, Window};

/// Separator placed before a token typed straight after a word.
const WORD_SEPARATOR: char = ' ';

/// The text to insert at byte `cursor` of `text` so `token` begins a new word.
pub(super) fn word_start_insertion(text: &str, cursor: usize, token: &str) -> String {
    let before = text.get(..cursor).unwrap_or(text);
    match before.chars().last() {
        Some(prev) if !prev.is_whitespace() => format!("{WORD_SEPARATOR}{token}"),
        _ => token.to_string(),
    }
}

/// Inserts `token` at the cursor of `input` (as a new word) and focuses the editor.
pub(super) fn insert_token(input: &Entity<TextareaState>, token: &str, window: &mut Window, cx: &mut App) {
    input.update(cx, |state, cx| {
        let insertion = word_start_insertion(&state.value(), state.cursor(), token);
        state.insert(insertion, window, cx);
        state.focus(window, cx);
    });
}

#[cfg(test)]
mod tests {
    use super::word_start_insertion;

    #[test]
    fn token_after_a_word_gets_a_separating_space() {
        assert_eq!(word_start_insertion("hello", 5, "#"), " #");
        assert_eq!(word_start_insertion("héllo wörld", "héllo".len(), "[["), " [[");
    }

    #[test]
    fn token_at_start_or_after_whitespace_is_inserted_bare() {
        assert_eq!(word_start_insertion("", 0, "#"), "#");
        assert_eq!(word_start_insertion("hello ", 6, "#"), "#");
        assert_eq!(word_start_insertion("line\n", 5, "#"), "#");
    }

    #[test]
    fn out_of_range_cursor_falls_back_to_the_text_end() {
        assert_eq!(word_start_insertion("abc", 99, "#"), " #");
    }
}

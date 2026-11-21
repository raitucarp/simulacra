//! Formatting a composer selection: wrap it in a mark's delimiters, or take them off again
//! when it is already wrapped (the toolbar buttons and their shortcuts toggle).
//!
//! - The delimiters hug the text: surrounding whitespace in the selection stays outside, so
//!   the result parses (`**word** `, not `**word **`).
//! - Already wrapped means delimiters just outside the selection or at its very ends. Stars
//!   are counted, so italic inside bold (`***x***`) toggles one mark and keeps the other.
//! - Marks never cross a line break, so a selection over several lines is formatted line by
//!   line; code over several lines becomes a fenced block instead.
//! - An empty selection inserts an empty pair with the cursor between the delimiters.

mod lines;
mod wrap;

use std::ops::Range;

/// A mark the composer can apply to a selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatMark {
    Bold,
    Italic,
    Underline,
    Code,
    Spoiler,
}

impl FormatMark {
    /// Every mark, in toolbar order.
    pub const ALL: [FormatMark; 5] = [Self::Bold, Self::Italic, Self::Underline, Self::Code, Self::Spoiler];

    /// The delimiter written on both sides of the text.
    pub const fn delimiter(self) -> &'static str {
        match self {
            Self::Bold => "**",
            Self::Italic => "*",
            Self::Underline => "__",
            Self::Code => "`",
            Self::Spoiler => "||",
        }
    }

    /// Stable id (locale keys `composer.format.<id>`, element ids).
    pub const fn id(self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::Underline => "underline",
            Self::Code => "code",
            Self::Spoiler => "spoiler",
        }
    }
}

/// One replacement in the text and the selection to show after it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatEdit {
    /// Byte range of the old text to replace.
    pub range: Range<usize>,
    /// Text replacing it.
    pub text: String,
    /// Byte range selected afterwards, in the new text.
    pub selection: Range<usize>,
}

impl FormatEdit {
    /// `text` with the edit applied (what the editor holds afterwards).
    pub fn apply(&self, text: &str) -> String {
        format!("{}{}{}", &text[..self.range.start], self.text, &text[self.range.end..])
    }
}

/// Toggles `mark` on the `selection` (byte offsets) of `text`.
pub fn toggle_mark(text: &str, selection: Range<usize>, mark: FormatMark) -> FormatEdit {
    let selection = clamp(text, selection);
    if text[selection.clone()].contains(lines::LINE_BREAK) {
        return lines::toggle_lines(text, selection, mark);
    }
    wrap::toggle_span(text, selection, mark)
}

/// `selection` ordered, inside `text` and on character boundaries.
fn clamp(text: &str, selection: Range<usize>) -> Range<usize> {
    let floor = |mut at: usize| {
        at = at.min(text.len());
        while !text.is_char_boundary(at) {
            at -= 1;
        }
        at
    };
    let (start, end) = (floor(selection.start), floor(selection.end));
    start.min(end)..start.max(end)
}

#[cfg(test)]
mod tests;

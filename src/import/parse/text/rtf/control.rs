//! Reading one RTF control sequence after a backslash.

use std::iter::Peekable;
use std::str::Chars;

/// What follows a backslash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Control {
    /// A control word with its optional numeric parameter (`\par`, `舒`, `\uc1`).
    Word(String, Option<i32>),
    /// A literal character (`\\`, `\{`, `\}`, `\~`, `\'e9`).
    Literal(char),
    /// `\*`: the group is an ignorable destination.
    Ignorable,
    /// A backslash before a line break, which is a paragraph break.
    Break,
    /// Anything else, ignored.
    Other,
}

/// A byte from `\'hh` decoded as Windows-1252.
fn code_page_char(byte: u8) -> char {
    encoding_rs::WINDOWS_1252.decode(&[byte]).0.chars().next().unwrap_or(char::REPLACEMENT_CHARACTER)
}

fn take_hex(chars: &mut Peekable<Chars>) -> Option<u8> {
    let high = chars.next()?.to_digit(16)?;
    let low = chars.next()?.to_digit(16)?;
    u8::try_from(high * 16 + low).ok()
}

fn take_word(chars: &mut Peekable<Chars>) -> Control {
    let mut word = String::new();
    while let Some(c) = chars.next_if(|c| c.is_ascii_alphabetic()) {
        word.push(c);
    }
    let mut digits = String::new();
    if let Some(sign) = chars.next_if_eq(&'-') {
        digits.push(sign);
    }
    while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
        digits.push(c);
    }
    chars.next_if_eq(&' ');
    Control::Word(word, digits.parse().ok())
}

/// Reads the control sequence after a backslash.
pub fn read_control(chars: &mut Peekable<Chars>) -> Control {
    match chars.peek().copied() {
        Some(c) if c.is_ascii_alphabetic() => take_word(chars),
        Some('\'') => {
            chars.next();
            take_hex(chars).map_or(Control::Other, |byte| Control::Literal(code_page_char(byte)))
        }
        Some(c @ ('\\' | '{' | '}')) => {
            chars.next();
            Control::Literal(c)
        }
        Some('~') => {
            chars.next();
            Control::Literal(' ')
        }
        Some('*') => {
            chars.next();
            Control::Ignorable
        }
        Some('\n' | '\r') => {
            chars.next();
            Control::Break
        }
        _ => {
            chars.next();
            Control::Other
        }
    }
}

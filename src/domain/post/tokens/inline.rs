//! Inline lexer: escapes, code spans, mark delimiters, web links, wikilinks and hashtags.
//!
//! Delimiters follow "flanking" rules: an opener must be followed by a non-space and a closer
//! preceded by one, so `2 * 3 * 4` and a `* ` list bullet at a line start stay literal.
//! Underscores also refuse to open or close inside a word (`snake__case__name`).

use super::atoms::{scan_hashtag, scan_url, scan_wikilink};
use super::builder::Builder;
use super::{BodyToken, Mark};

/// Escape character: the next markup character is taken literally.
const ESCAPE: char = '\\';
/// Characters a backslash can escape.
const ESCAPABLE: [char; 8] = ['\\', '*', '_', '`', '|', '#', '[', ']'];
const STAR: char = '*';
const UNDERSCORE: char = '_';
const PIPE: char = '|';
const BACKTICK: char = '`';
const LINE_BREAK: char = '\n';

/// Parses one prose segment (no fenced blocks) into inline tokens.
pub(super) fn parse_inline(text: &str) -> Vec<BodyToken> {
    let chars: Vec<char> = text.chars().collect();
    let mut builder = Builder::new();
    let mut i = 0;
    while i < chars.len() {
        i = step(&chars, i, &mut builder);
    }
    builder.finish()
}

/// Consumes the construct starting at `i`; returns the index after it.
fn step(chars: &[char], i: usize, builder: &mut Builder) -> usize {
    match chars[i] {
        LINE_BREAK => {
            builder.line_break();
            i + 1
        }
        ESCAPE if chars.get(i + 1).is_some_and(|next| ESCAPABLE.contains(next)) => {
            builder.char(chars[i + 1]);
            i + 2
        }
        BACKTICK => code_span(chars, i, builder),
        STAR | UNDERSCORE | PIPE => delimiter_run(chars, i, builder),
        c => match scan_url(chars, i).or_else(|| scan_wikilink(chars, i)).or_else(|| scan_hashtag(chars, i)) {
            Some((token, next)) => {
                builder.atom(token);
                next
            }
            None => {
                builder.char(c);
                i + 1
            }
        },
    }
}

/// Length of the run of `c` starting at `i`.
fn run_len(chars: &[char], i: usize, c: char) -> usize {
    chars[i..].iter().take_while(|&&x| x == c).count()
}

/// A run of N backticks opens a code span closed by the next run of exactly N on the line.
fn code_span(chars: &[char], i: usize, builder: &mut Builder) -> usize {
    let run = run_len(chars, i, BACKTICK);
    let start = i + run;
    let mut j = start;
    while j < chars.len() && chars[j] != LINE_BREAK {
        if chars[j] != BACKTICK {
            j += 1;
            continue;
        }
        let closing = run_len(chars, j, BACKTICK);
        if closing == run {
            builder.atom(BodyToken::Code(chars[start..j].iter().collect()));
            return j + run;
        }
        j += closing;
    }
    builder.text(&chars[i..start].iter().collect::<String>());
    start
}

/// Lexes a run of `*`, `_` or `|` into mark delimiters (or literal text).
fn delimiter_run(chars: &[char], i: usize, builder: &mut Builder) -> usize {
    let c = chars[i];
    let run = run_len(chars, i, c);
    let end = i + run;
    let prev = i.checked_sub(1).map(|p| chars[p]);
    let marks: &[Mark] = match (c, run) {
        (STAR, 1) => &[Mark::Italic],
        (STAR, 2) => &[Mark::Bold],
        (STAR, 3) if builder.top_mark() == Some(Mark::Italic) => &[Mark::Italic, Mark::Bold],
        (STAR, 3) => &[Mark::Bold, Mark::Italic],
        (UNDERSCORE, 2) => &[Mark::Underline],
        (PIPE, 2) => &[Mark::Spoiler],
        _ => &[],
    };
    if marks.is_empty() {
        builder.text(&chars[i..end].iter().collect::<String>());
        return end;
    }
    let (can_open, can_close) = flanking(c, prev, chars.get(end).copied());
    for mark in marks {
        builder.delimiter(*mark, can_open, can_close);
    }
    end
}

/// Whether a delimiter run between `prev` and `next` may open and may close a mark.
fn flanking(c: char, prev: Option<char>, next: Option<char>) -> (bool, bool) {
    let mut can_open = next.is_some_and(|n| !n.is_whitespace());
    let mut can_close = prev.is_some_and(|p| !p.is_whitespace());
    if c == UNDERSCORE {
        can_open &= prev.is_none_or(|p| !p.is_alphanumeric());
        can_close &= next.is_none_or(|n| !n.is_alphanumeric());
    }
    (can_open, can_close)
}

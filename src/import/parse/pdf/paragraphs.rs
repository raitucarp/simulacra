//! Rebuilding paragraphs from extracted lines with layout-free heuristics: a line that ends a
//! sentence and stops short of the page's typical line length ends its paragraph; a line
//! that is much shorter (a heading, a signature) stands alone; hyphenated breaks are mended.

/// A line shorter than this share of the typical length, ending a sentence, ends a paragraph.
const SHORT_LINE_SHARE: f32 = 0.8;
/// A line shorter than this share of the typical length stands alone.
const STANDALONE_SHARE: f32 = 0.45;
/// Typical line length = this percentile of the line lengths.
const TYPICAL_PERCENTILE: f32 = 0.75;
const SENTENCE_ENDINGS: [char; 9] = ['.', '!', '?', ':', '…', '"', '”', '»', '。'];
const BULLETS: [&str; 4] = ["• ", "- ", "– ", "* "];

fn char_len(text: &str) -> usize {
    text.chars().count()
}

/// The length a full line of body text has on this page.
fn typical_length(lines: &[String]) -> usize {
    let mut lengths: Vec<usize> = lines.iter().map(|line| char_len(line)).filter(|length| *length > 0).collect();
    lengths.sort_unstable();
    let at = ((lengths.len() as f32 - 1.0) * TYPICAL_PERCENTILE).round() as usize;
    lengths.get(at).copied().unwrap_or(0)
}

fn starts_item(line: &str) -> bool {
    BULLETS.iter().any(|bullet| line.starts_with(bullet))
}

/// Whether the paragraph ends after `line`.
fn ends_paragraph(line: &str, typical: usize) -> bool {
    let length = char_len(line) as f32;
    let ends_sentence = line.trim_end().ends_with(SENTENCE_ENDINGS);
    length < typical as f32 * STANDALONE_SHARE || (ends_sentence && length < typical as f32 * SHORT_LINE_SHARE)
}

/// Appends `line` to `paragraph`, mending `exam-\nple` into `example`.
fn join_line(paragraph: &mut String, line: &str) {
    let next_is_lower = line.chars().next().is_some_and(char::is_lowercase);
    let hyphenated = paragraph.ends_with('-') && paragraph.chars().rev().nth(1).is_some_and(char::is_alphabetic);
    if hyphenated && next_is_lower {
        paragraph.pop();
    } else if !paragraph.is_empty() {
        paragraph.push(' ');
    }
    paragraph.push_str(line);
}

/// Moves the paragraph being built (if any) to `paragraphs`.
fn flush(paragraphs: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        paragraphs.push(std::mem::take(current));
    }
}

/// Paragraphs of one page, from its lines in reading order; a blank line always breaks.
pub fn rebuild_paragraphs(lines: &[String]) -> Vec<String> {
    let typical = typical_length(lines);
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    for line in lines.iter().map(|line| line.trim()) {
        if line.is_empty() || starts_item(line) {
            flush(&mut paragraphs, &mut current);
        }
        if line.is_empty() {
            continue;
        }
        join_line(&mut current, line);
        if ends_paragraph(line, typical) {
            flush(&mut paragraphs, &mut current);
        }
    }
    flush(&mut paragraphs, &mut current);
    paragraphs
}

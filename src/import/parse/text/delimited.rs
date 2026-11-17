//! CSV and TSV: quoted fields with doubled quotes, one block per data row written as
//! `column: value` pairs from the header row.

use crate::import::document::Block;
use crate::import::parse::sections::normalize_whitespace;
use crate::import::parse::Content;
use crate::import::document::Section;

const QUOTE: char = '"';
/// Separator between the `column: value` pairs of a row.
const PAIR_SEPARATOR: &str = " · ";

/// The rows of delimited text; quoted fields may span lines.
pub fn rows(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let (mut row, mut field, mut quoted) = (Vec::new(), String::new(), false);
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            (QUOTE, true) if chars.peek() == Some(&QUOTE) => {
                field.push(QUOTE);
                chars.next();
            }
            (QUOTE, _) => quoted = !quoted,
            (c, false) if c == delimiter => row.push(std::mem::take(&mut field)),
            ('\n', false) => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            ('\r', false) => {}
            (c, _) => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows.into_iter().filter(|row| row.iter().any(|field| !field.trim().is_empty())).collect()
}

/// One row as `column: value` pairs (blank values skipped).
fn describe_row(header: &[String], row: &[String]) -> String {
    let pairs = row.iter().enumerate().filter(|(_, value)| !value.trim().is_empty()).map(|(index, value)| {
        let value = normalize_whitespace(value);
        match header.get(index).map(|name| normalize_whitespace(name)).filter(|name| !name.is_empty()) {
            Some(name) => format!("{name}: {value}"),
            None => value,
        }
    });
    pairs.collect::<Vec<_>>().join(PAIR_SEPARATOR)
}

/// Parses delimited text: the first row names the columns, each later row is a block.
pub fn parse_delimited(text: &str, delimiter: char) -> Content {
    let mut all = rows(text, delimiter).into_iter();
    let header = all.next().unwrap_or_default();
    let blocks: Vec<Block> = all.map(|row| Block::paragraph(describe_row(&header, &row))).collect();
    let sections = if blocks.is_empty() { Vec::new() } else { vec![Section::new(None, blocks)] };
    Content { sections, ..Content::default() }
}

//! The text layer of one PDF page as lines: text-showing operators are decoded with the page
//! fonts' encodings, and line moves (`Td`, `TD`, `Tm`, `T*`, `'`, `"`, `ET`) end a line. A move
//! over an empty line, or one clearly wider than the usual leading, leaves a blank line (`""`),
//! which marks a paragraph break.

use lopdf::content::Content;
use lopdf::{Document, Encoding, Object, ObjectId};
use std::collections::BTreeMap;

/// Most decompressed content per page (bytes).
const MAX_PAGE_CONTENT_BYTES: usize = 32 * 1024 * 1024;
/// A `TJ` adjustment (thousandths of an em) at or below this reads as a word space.
const WORD_GAP_ADJUSTMENT: f32 = -180.0;
/// A line move this many times the usual leading is a paragraph gap.
const PARAGRAPH_GAP_FACTOR: f32 = 1.4;

/// Builds lines from the operations.
#[derive(Default)]
struct LineCollector {
    lines: Vec<String>,
    current: String,
    /// The smallest move that ended a line of text (the body leading).
    leading: Option<f32>,
    /// The next move positions a new text object rather than moving down a line.
    fresh_object: bool,
}

impl LineCollector {
    fn end_line(&mut self) {
        let line = std::mem::take(&mut self.current);
        let line = line.trim();
        if !line.is_empty() {
            self.lines.push(line.to_string());
        }
    }

    /// A blank-line marker, never doubled and never first.
    fn gap(&mut self) {
        if self.lines.last().is_some_and(|line| !line.is_empty()) {
            self.lines.push(String::new());
        }
    }

    /// A move down by `dy` (positive).
    fn move_down(&mut self, dy: f32) {
        let blank = self.current.trim().is_empty();
        self.end_line();
        if std::mem::take(&mut self.fresh_object) {
            return;
        }
        if blank {
            return self.gap();
        }
        // The leading is learned from moves that end a line of text, never from empty moves.
        let wide = self.leading.is_some_and(|leading| dy > leading * PARAGRAPH_GAP_FACTOR);
        self.leading = Some(self.leading.map_or(dy, |leading| leading.min(dy)));
        if wide {
            self.gap();
        }
    }

    fn space(&mut self) {
        if !self.current.ends_with(' ') && !self.current.is_empty() {
            self.current.push(' ');
        }
    }

    fn show(&mut self, encoding: Option<&Encoding>, operand: &Object) {
        match operand {
            Object::String(bytes, _) => self.current.push_str(&decode(encoding, bytes)),
            Object::Array(items) => items.iter().for_each(|item| match item {
                Object::Integer(_) | Object::Real(_) if item.as_float().unwrap_or(0.0) <= WORD_GAP_ADJUSTMENT => self.space(),
                other => self.show(encoding, other),
            }),
            _ => {}
        }
    }
}

fn decode(encoding: Option<&Encoding>, bytes: &[u8]) -> String {
    encoding
        .and_then(|encoding| encoding.bytes_to_string(bytes).ok())
        .unwrap_or_else(|| bytes.iter().map(|&b| char::from(b)).collect())
}

/// The vertical offset of a `Td`/`TD` move, when it goes to another line.
fn vertical_move(operands: &[Object]) -> Option<f32> {
    operands.get(1).and_then(|y| y.as_float().ok()).map(f32::abs).filter(|y| *y > f32::EPSILON)
}

/// The page's text lines, in content order. Unreadable pages yield no lines.
pub fn page_lines(doc: &Document, page_id: ObjectId) -> Vec<String> {
    let encodings: BTreeMap<Vec<u8>, Encoding> = doc
        .get_page_fonts(page_id)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(name, font)| font.get_font_encoding(doc).ok().map(|encoding| (name, encoding)))
        .collect();
    let Ok(data) = doc.get_page_content_with_limit(page_id, MAX_PAGE_CONTENT_BYTES) else { return Vec::new() };
    let Ok(content) = Content::decode(&data) else { return Vec::new() };
    let mut collector = LineCollector::default();
    let mut font: Option<&Encoding> = None;
    let mut matrix_y: Option<f32> = None;
    for operation in &content.operations {
        let operands = &operation.operands;
        match operation.operator.as_str() {
            "Tf" => font = operands.first().and_then(|name| name.as_name().ok()).and_then(|name| encodings.get(name)),
            "Tj" | "TJ" => operands.iter().for_each(|operand| collector.show(font, operand)),
            "'" | "\"" => {
                collector.end_line();
                operands.last().into_iter().for_each(|operand| collector.show(font, operand));
            }
            "Td" | "TD" => match vertical_move(operands) {
                Some(dy) => collector.move_down(dy),
                None => collector.space(),
            },
            "BT" => collector.fresh_object = true,
            "Tm" => {
                // A new text matrix on the same baseline continues the line.
                let y = operands.get(5).and_then(|y| y.as_float().ok());
                if y.is_some() && y == matrix_y { collector.space() } else { collector.end_line() }
                matrix_y = y;
            }
            "T*" | "ET" => collector.end_line(),
            _ => {}
        }
    }
    collector.end_line();
    collector.lines
}

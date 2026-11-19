//! Review items as CSV (RFC 4180: comma-separated, CRLF rows, fields quoted when they hold a
//! comma, a quote or a line break, quotes doubled).

use crate::domain::review::{ReviewItem, Subject};

const SEPARATOR: &str = ",";
const ROW_END: &str = "\r\n";
const QUOTE: char = '"';
const TAG_SEPARATOR: &str = " ";
const HEADER: [&str; 11] = ["id", "type", "anchor", "written", "tags", "created", "due", "interval_days", "reps", "lapses", "suspended"];

/// `value` as one CSV field.
pub fn csv_field(value: &str) -> String {
    let needs_quotes = value.contains([',', QUOTE, '\n', '\r']);
    if needs_quotes {
        format!("{QUOTE}{}{QUOTE}", value.replace(QUOTE, "\"\""))
    } else {
        value.to_string()
    }
}

fn row(fields: &[String]) -> String {
    let joined: Vec<String> = fields.iter().map(|f| csv_field(f)).collect();
    format!("{}{ROW_END}", joined.join(SEPARATOR))
}

fn item_row(item: &ReviewItem) -> String {
    let (anchor, written) = match &item.subject {
        Subject::Anchored { anchor, written, .. } => (anchor.key(), written.clone()),
        Subject::Connections { map } => (String::new(), map.title.clone()),
    };
    let schedule = &item.schedule;
    row(&[
        item.id.as_str().to_string(),
        item.subject.key().to_string(),
        anchor,
        written,
        item.tags.join(TAG_SEPARATOR),
        crate::time::rfc3339(item.created_at),
        schedule.due.to_string(),
        schedule.interval.to_string(),
        schedule.reps.to_string(),
        schedule.lapses.to_string(),
        item.suspended.to_string(),
    ])
}

/// Every item as a CSV table with a header row.
pub fn review_csv(items: &[ReviewItem]) -> String {
    let header = row(&HEADER.map(str::to_string));
    std::iter::once(header).chain(items.iter().map(item_row)).collect()
}

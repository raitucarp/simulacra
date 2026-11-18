//! Cloze markup of the retired flashcards (`{{c1::answer}}` or `{{c1::answer::hint}}`),
//! read only to carry old cards over: the hidden words become the question written on a
//! passage, the full text becomes the passage.

const OPEN: &str = "{{c";
const SEPARATOR: &str = "::";
const CLOSE: &str = "}}";
/// A hidden deletion in a prompt.
const BLANK: &str = "…";

/// A piece of a cloze text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClozePart {
    Plain(String),
    Deletion { ordinal: u32, answer: String, hint: Option<String> },
}

/// Parses one `{{cN::…}}` body (`N::answer[::hint]`) after the opening `{{c`.
fn parse_deletion(body: &str) -> Option<ClozePart> {
    let (number, rest) = body.split_once(SEPARATOR)?;
    let ordinal = number.trim().parse::<u32>().ok().filter(|n| *n > 0)?;
    let (answer, hint) = match rest.split_once(SEPARATOR) {
        Some((answer, hint)) => (answer, Some(hint.trim().to_string()).filter(|h| !h.is_empty())),
        None => (rest, None),
    };
    (!answer.trim().is_empty()).then(|| ClozePart::Deletion { ordinal, answer: answer.to_string(), hint })
}

/// The parts of `text`, in order; malformed markup stays plain text.
pub fn parse_cloze(text: &str) -> Vec<ClozePart> {
    let mut parts = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        let Some(end) = after.find(CLOSE) else { break };
        let Some(deletion) = parse_deletion(&after[..end]) else {
            push_plain(&mut parts, &rest[..start + OPEN.len()]);
            rest = after;
            continue;
        };
        push_plain(&mut parts, &rest[..start]);
        parts.push(deletion);
        rest = &after[end + CLOSE.len()..];
    }
    push_plain(&mut parts, rest);
    parts
}

fn push_plain(parts: &mut Vec<ClozePart>, text: &str) {
    if text.is_empty() {
        return;
    }
    match parts.last_mut() {
        Some(ClozePart::Plain(previous)) => previous.push_str(text),
        _ => parts.push(ClozePart::Plain(text.to_string())),
    }
}

/// The text with every deletion replaced by its answer.
pub fn plain_text(text: &str) -> String {
    parse_cloze(text)
        .into_iter()
        .map(|part| match part {
            ClozePart::Plain(text) => text,
            ClozePart::Deletion { answer, .. } => answer,
        })
        .collect()
}

/// The prompt of card `ordinal`: its deletions blanked (with their hint), the others shown.
pub fn prompt_text(text: &str, ordinal: u32) -> String {
    parse_cloze(text)
        .into_iter()
        .map(|part| match part {
            ClozePart::Plain(text) => text,
            ClozePart::Deletion { ordinal: n, hint: Some(hint), .. } if n == ordinal => format!("{BLANK} ({hint})"),
            ClozePart::Deletion { ordinal: n, .. } if n == ordinal => BLANK.to_string(),
            ClozePart::Deletion { answer, .. } => answer,
        })
        .collect()
}

//! Inline markdown of agent output: links, images, autolinks, strikethrough and `<br>`;
//! code spans, escapes and `[[wikilinks]]` are copied untouched.

const BACKTICK: char = '`';
const ESCAPE: char = '\\';
const STRIKE: &str = "~~";
const WIKILINK_OPEN: &str = "[[";
/// Marker before the link brackets of an image (`![alt](src)`).
const IMAGE_MARKER: char = '!';
const LINK_OPEN: char = '[';
const LINK_MIDDLE: &str = "](";
const LINK_CLOSE: char = ')';
const AUTOLINK_OPEN: &str = "<http";
const AUTOLINK_CLOSE: char = '>';
const LINE_BREAK_TAGS: [&str; 3] = ["<br>", "<br/>", "<br />"];
const LINE_BREAK: char = '\n';

/// `text` with unsupported inline markdown reduced to what a reader sees.
pub(super) fn sanitize_inline(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        let (replacement, len) = construct(rest, c).unwrap_or_else(|| (c.to_string(), c.len_utf8()));
        out.push_str(&replacement);
        rest = &rest[len..];
    }
    out
}

/// The text a construct at the start of `rest` becomes, and its length in bytes.
fn construct(rest: &str, c: char) -> Option<(String, usize)> {
    if c == BACKTICK {
        let len = code_span_len(rest);
        return Some((rest[..len].to_string(), len));
    }
    if c == ESCAPE {
        let len = c.len_utf8() + rest[1..].chars().next().map_or(0, char::len_utf8);
        return Some((rest[..len].to_string(), len));
    }
    if rest.starts_with(WIKILINK_OPEN) {
        return Some((WIKILINK_OPEN.to_string(), WIKILINK_OPEN.len()));
    }
    if rest.starts_with(STRIKE) {
        return Some((String::new(), STRIKE.len()));
    }
    if let Some(tag) = LINE_BREAK_TAGS.iter().find(|tag| rest.get(..tag.len()).is_some_and(|head| head.eq_ignore_ascii_case(tag))) {
        return Some((LINE_BREAK.to_string(), tag.len()));
    }
    image(rest).or_else(|| link(rest)).or_else(|| autolink(rest))
}

/// A code span copied whole (an unclosed run of backticks is copied as it is).
fn code_span_len(rest: &str) -> usize {
    let run = rest.chars().take_while(|&c| c == BACKTICK).count();
    let fence = &rest[..run];
    let mut at = run;
    while let Some(found) = rest[at..].find(fence) {
        let start = at + found;
        let closing = rest[start..].chars().take_while(|&c| c == BACKTICK).count();
        if closing == run {
            return start + run;
        }
        at = start + closing;
    }
    run
}

/// `[text](url)` split into its text and address, with the length it spans.
fn link_parts(rest: &str) -> Option<(&str, &str, usize)> {
    let body = rest.strip_prefix(LINK_OPEN)?;
    let middle = body.find(LINK_MIDDLE)?;
    let text = &body[..middle];
    let after = &body[middle + LINK_MIDDLE.len()..];
    let close = after.find(LINK_CLOSE)?;
    let url = after[..close].split_whitespace().next().unwrap_or_default();
    (!text.contains(LINK_OPEN)).then_some((text, url, 1 + middle + LINK_MIDDLE.len() + close + 1))
}

/// `![alt](src)` becomes its description.
fn image(rest: &str) -> Option<(String, usize)> {
    let (alt, _, len) = link_parts(rest.strip_prefix(IMAGE_MARKER)?)?;
    Some((alt.trim().to_string(), len + 1))
}

/// `[text](url)` becomes `text (url)`, or just the address when the text repeats it.
fn link(rest: &str) -> Option<(String, usize)> {
    let (text, url, len) = link_parts(rest)?;
    let text = text.trim();
    let shown = if text.is_empty() || text == url { url.to_string() } else { format!("{text} ({url})") };
    Some((shown, len))
}

/// `<https://...>` becomes the bare address.
fn autolink(rest: &str) -> Option<(String, usize)> {
    if !rest.starts_with(AUTOLINK_OPEN) {
        return None;
    }
    let close = rest.find(AUTOLINK_CLOSE)?;
    let url = &rest[1..close];
    (!url.contains(char::is_whitespace)).then(|| (url.to_string(), close + 1))
}

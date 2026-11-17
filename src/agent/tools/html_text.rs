//! A small HTML → text converter for fetched pages (no DOM, no scripts).

/// Elements whose content is never text.
const HIDDEN_ELEMENTS: [&str; 5] = ["script", "style", "noscript", "svg", "head"];
/// Tags that end a line of text.
const BLOCK_TAGS: [&str; 14] =
    ["p", "div", "br", "li", "h1", "h2", "h3", "h4", "h5", "h6", "tr", "section", "article", "blockquote"];
/// Named entities decoded (others are left as-is).
const ENTITIES: [(&str, &str); 7] =
    [("&nbsp;", " "), ("&lt;", "<"), ("&gt;", ">"), ("&quot;", "\""), ("&#39;", "'"), ("&apos;", "'"), ("&amp;", "&")];

/// Visible text of `html`: hidden elements dropped, tags removed, block tags turned into
/// line breaks, common entities decoded and whitespace collapsed.
pub fn html_to_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len() / 2);
    let mut rest = html;
    while let Some(open) = rest.find('<') {
        out.push_str(&rest[..open]);
        let tag_and_after = &rest[open + 1..];
        let Some(close) = tag_and_after.find('>') else { break };
        let tag = tag_name(&tag_and_after[..close]);
        rest = &tag_and_after[close + 1..];
        if HIDDEN_ELEMENTS.contains(&tag.as_str()) {
            rest = skip_element(rest, &tag);
        } else if BLOCK_TAGS.contains(&tag.as_str()) {
            out.push('\n');
        } else {
            out.push(' ');
        }
    }
    if !rest.contains('<') {
        out.push_str(rest);
    }
    collapse(&decode_entities(&out))
}

/// Lowercased element name of a tag body (`/p class=x` → `p`).
fn tag_name(tag: &str) -> String {
    tag.trim_start_matches('/')
        .split(|c: char| c.is_whitespace() || c == '/')
        .next()
        .unwrap_or_default()
        .to_lowercase()
}

/// `rest` after the closing tag of `tag` (or empty when unclosed).
fn skip_element<'a>(rest: &'a str, tag: &str) -> &'a str {
    let closing = format!("</{tag}");
    let lower = rest.to_ascii_lowercase();
    match lower.find(&closing) {
        Some(at) => rest[at..].find('>').map_or("", |end| &rest[at + end + 1..]),
        None => "",
    }
}

fn decode_entities(text: &str) -> String {
    ENTITIES.iter().fold(text.to_string(), |acc, (entity, plain)| acc.replace(entity, plain))
}

/// Trims every line, collapses inner whitespace and drops empty lines.
fn collapse(text: &str) -> String {
    text.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

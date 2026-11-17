//! Character references: the XML five, the common HTML typographic names and numeric
//! references. Unknown names are left as written.

use std::borrow::Cow;

/// Longest entity name looked up (`&thinsp;`).
const MAX_NAME_LEN: usize = 8;

const NAMED: [(&str, char); 30] = [
    ("amp", '&'), ("lt", '<'), ("gt", '>'), ("quot", '"'), ("apos", '\''), ("nbsp", '\u{a0}'),
    ("mdash", '—'), ("ndash", '–'), ("hellip", '…'), ("lsquo", '‘'), ("rsquo", '’'), ("ldquo", '“'),
    ("rdquo", '”'), ("laquo", '«'), ("raquo", '»'), ("copy", '©'), ("reg", '®'), ("shy", '\u{ad}'),
    ("thinsp", '\u{2009}'), ("ensp", '\u{2002}'), ("emsp", '\u{2003}'), ("eacute", 'é'), ("egrave", 'è'),
    ("agrave", 'à'), ("ccedil", 'ç'), ("uuml", 'ü'), ("ouml", 'ö'), ("auml", 'ä'), ("szlig", 'ß'),
    ("middot", '·'),
];

/// `text` with every recognised character reference replaced.
pub fn decode_entities(text: &str) -> Cow<'_, str> {
    if !text.contains('&') {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        match reference(rest) {
            Some((c, used)) => {
                out.push(c);
                rest = &rest[used..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// The character of the reference starting `text` (at `&`) and the bytes it spans.
fn reference(text: &str) -> Option<(char, usize)> {
    let end = text[1..].find(';').filter(|&end| end <= MAX_NAME_LEN + 1)? + 1;
    let name = &text[1..end];
    let c = match name.strip_prefix('#') {
        Some(hex) if hex.starts_with(['x', 'X']) => u32::from_str_radix(&hex[1..], 16).ok().and_then(char::from_u32),
        Some(decimal) => decimal.parse::<u32>().ok().and_then(char::from_u32),
        None => NAMED.iter().find(|(key, _)| *key == name).map(|(_, c)| *c),
    }?;
    Some((c, end + 1))
}

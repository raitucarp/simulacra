//! Small queries over markup: the text of every element with a name, and its opening tags.

use super::{decode_entities, local_name, tokenize, Tag, Token};
use crate::import::parse::normalize_whitespace;

/// The normalised text of every (outermost) element whose local name is `name`, in order;
/// empty elements are skipped.
pub fn element_texts(src: &str, name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    let mut nested = 0usize;
    for token in tokenize(src) {
        match token {
            Token::Open(tag) if tag.is(name) && !tag.self_closing => match current {
                Some(_) => nested += 1,
                None => current = Some(String::new()),
            },
            Token::Close(raw) if local_name(raw.trim()).eq_ignore_ascii_case(name) => {
                if nested > 0 {
                    nested -= 1;
                } else if let Some(text) = current.take().map(|text| normalize_whitespace(&text)) {
                    out.extend((!text.is_empty()).then_some(text));
                }
            }
            Token::Text(text) => {
                if let Some(buffer) = current.as_mut() {
                    buffer.push_str(&decode_entities(text));
                }
            }
            _ => {}
        }
    }
    out
}

/// The first such text.
pub fn first_element_text(src: &str, name: &str) -> Option<String> {
    element_texts(src, name).into_iter().next()
}

/// Every opening tag whose local name is `name`.
pub fn tags_named<'a>(src: &'a str, name: &'a str) -> impl Iterator<Item = Tag<'a>> + 'a {
    tokenize(src).filter_map(move |token| match token {
        Token::Open(tag) if tag.is(name) => Some(tag),
        _ => None,
    })
}

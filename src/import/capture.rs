//! Quick capture (pure): what a dumped piece of text is. A lone URL becomes a link post, a
//! URL with words around it a note with the link attached, anything else a note.

const URL_SCHEMES: [&str; 2] = ["https://", "http://"];
/// Characters that end a URL found inside text.
const URL_TRAILING: [char; 6] = ['.', ',', ';', ')', '!', '?'];

/// A classified capture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capture {
    Empty,
    /// Text alone.
    Note(String),
    /// A link, with the words around it (possibly none).
    Link { url: String, note: String },
}

fn is_url(word: &str) -> bool {
    URL_SCHEMES.iter().any(|scheme| word.starts_with(scheme) && word.len() > scheme.len())
}

/// The first URL in `text`, without trailing punctuation.
pub fn first_url(text: &str) -> Option<String> {
    text.split_whitespace().find(|word| is_url(word)).map(|word| word.trim_end_matches(URL_TRAILING).to_string())
}

/// Classifies captured text.
pub fn classify(text: &str) -> Capture {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Capture::Empty;
    }
    match first_url(trimmed) {
        Some(url) => {
            let note = if trimmed.split_whitespace().count() == 1 { String::new() } else { trimmed.to_string() };
            Capture::Link { url, note }
        }
        None => Capture::Note(trimmed.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_are_notes_links_or_nothing() {
        assert_eq!(classify("   \n"), Capture::Empty);
        assert_eq!(classify(" an idea \n"), Capture::Note("an idea".into()));
        assert_eq!(classify("https://example.org/a?b=1"), Capture::Link { url: "https://example.org/a?b=1".into(), note: String::new() });
        assert_eq!(
            classify("Read this: https://example.org/essay. Later."),
            Capture::Link { url: "https://example.org/essay".into(), note: "Read this: https://example.org/essay. Later.".into() }
        );
        assert_eq!(classify("http:// alone"), Capture::Note("http:// alone".into()));
    }
}

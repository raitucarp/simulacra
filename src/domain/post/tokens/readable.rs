//! A post on one line as a reader would say it: no markup, no `[[post_…]]` links or
//! embeds (their ids mean nothing out of context), each to-do reduced to its words.
//! List views (rapid log, board, garden, series…) and banners quoting a post use it.

use super::{tokenize_post_body, BodyToken};
use crate::domain::todo::parse_todos;

/// Wikilinks to posts start with this; links to anything else keep their target as text.
const POST_LINK_PREFIX: &str = "post_";
/// Lines that open or close a fenced code block.
const FENCE: &str = "```";
/// Between two to-dos, or a to-do and other text, on the one line.
const TODO_JOIN: &str = " · ";

/// `content` on one line: markup and links to posts left out, to-dos as their words.
pub fn readable_line(content: &str) -> String {
    let mut out = String::new();
    let mut after_todo = false;
    for line in content.lines().filter(|line| !line.trim_start().starts_with(FENCE)) {
        let todo = parse_todos(line).into_iter().next();
        let words = readable_words(todo.as_ref().map_or(line, |item| item.text.as_str()));
        if words.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push_str(if todo.is_some() || after_todo { TODO_JOIN } else { " " });
        }
        out.push_str(&words);
        after_todo = todo.is_some();
    }
    out
}

fn readable_words(text: &str) -> String {
    let mut out = String::new();
    tokenize_post_body(text).iter().for_each(|token| push_readable(token, &mut out));
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn push_readable(token: &BodyToken, out: &mut String) {
    match token {
        BodyToken::Text(text) | BodyToken::Code(text) | BodyToken::CodeBlock { code: text, .. } => out.push_str(text),
        BodyToken::Wikilink { target, trailing } if target.trim().starts_with(POST_LINK_PREFIX) => out.push_str(trailing),
        BodyToken::Embed { trailing, .. } => out.push_str(trailing),
        BodyToken::Wikilink { target: text, trailing }
        | BodyToken::Hashtag { tag: text, trailing }
        | BodyToken::Mention { handle: text, trailing }
        | BodyToken::Url { url: text, trailing } => {
            out.push_str(text);
            out.push_str(trailing);
        }
        BodyToken::Bold(children) | BodyToken::Italic(children) | BodyToken::Underline(children) | BodyToken::Spoiler(children) => {
            children.iter().for_each(|child| push_readable(child, out));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::readable_line;

    #[test]
    fn links_to_posts_and_markup_leave_the_line() {
        let line = readable_line("Is prediction **isomorphic** to modelling? [[post_2]] #Semiotics\n\nA second thought.");
        assert_eq!(line, "Is prediction isomorphic to modelling? #Semiotics A second thought.");
    }

    #[test]
    fn todos_keep_only_their_words() {
        let line = readable_line("Tasks for this week\n[ ] finish the draft ^fri !! &writing ~2h\n[x] import the Tractatus");
        assert_eq!(line, "Tasks for this week · finish the draft · import the Tractatus");
    }

    #[test]
    fn a_link_to_a_named_note_stays_readable() {
        assert_eq!(readable_line("See [[Semiotics]]."), "See Semiotics.");
    }
}

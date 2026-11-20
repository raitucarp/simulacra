//! Tests of the body token parser: inline marks, fenced blocks, generated inputs, lengths.

use super::*;

mod fences;
mod inline;
mod legacy;
mod props;
mod visible_length;

fn text(value: &str) -> BodyToken {
    BodyToken::Text(value.to_string())
}

fn bold(children: Vec<BodyToken>) -> BodyToken {
    BodyToken::Bold(children)
}

fn italic(children: Vec<BodyToken>) -> BodyToken {
    BodyToken::Italic(children)
}

fn code_block(lang: Option<&str>, code: &str) -> BodyToken {
    BodyToken::CodeBlock { lang: lang.map(str::to_string), code: code.to_string() }
}

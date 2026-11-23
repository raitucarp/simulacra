//! N17: the glossary view: every term the posts in view define, in alphabetical order, each
//! with its definition; a click opens the post that defines it. A locked post defines nothing,
//! so it is never listed.

use super::common::{empty_note, scroll_column};
use super::input::ViewInput;
use crate::domain::post::glossary::{Glossary, GlossaryEntry};
use gpui_kit::*;

pub(super) fn render_glossary(input: &ViewInput<'_>) -> AnyElement {
    let glossary = Glossary::from_posts(input.posts.iter().copied());
    if glossary.is_empty() {
        return empty_note("views.glossary.empty", input.theme);
    }
    scroll_column(input, "glossary").children(glossary.entries().into_iter().map(|entry| render_entry(entry, input))).into_any_element()
}

fn render_entry(entry: &GlossaryEntry, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let term = div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(entry.term.clone());
    let text = div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(entry.text.clone());
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("glossary-{}", entry.post)))
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |r| r.bg(hover))
        .on_click(input.opener(&entry.post))
        .child(term)
        .child(text)
}

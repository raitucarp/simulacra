//! N17: defined terms in the body flow. Each run of words is matched against the glossary
//! (the longest term first); a word that is part of a term is underlined, and hovering it
//! shows the term and its definition.

use super::layout::Item;
use super::BodyLinks;
use crate::domain::post::glossary::GlossaryEntry;
use crate::theme::ThemeTokens;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;
use std::rc::Rc;

/// For each item, the term it is part of (only words can be).
pub(super) fn marks(items: &[Item<'_>], links: &BodyLinks) -> Vec<Option<Rc<GlossaryEntry>>> {
    let mut marks = vec![None; items.len()];
    let Some(glossary) = links.glossary.as_ref().filter(|glossary| !glossary.is_empty()) else { return marks };
    let mut start = 0;
    while start < items.len() {
        let run: Vec<&str> = items[start..].iter().map_while(|item| if let Item::Word { text, .. } = item { Some(text.as_str()) } else { None }).collect();
        for (offset, mark) in glossary.mark_words(&run).into_iter().enumerate() {
            marks[start + offset] = mark;
        }
        start += run.len().max(1);
    }
    marks
}

/// `element`, underlined with its definition on hover when it is part of a term.
pub(super) fn defined(element: AnyElement, index: usize, mark: Option<&Rc<GlossaryEntry>>, links: &BodyLinks, theme: &ThemeTokens) -> AnyElement {
    let Some(entry) = mark.cloned() else { return element };
    div()
        .id(SharedString::from(format!("glossary-{}-{index}", links.scope)))
        .border_b(theme.border_width_hairline)
        .border_color(theme.accent_brand)
        .tooltip(move |window, cx| Tooltip::new(format!("{}: {}", entry.term, entry.text)).build(window, cx))
        .child(element)
        .into_any_element()
}

//! References dropdown of the action bar: quoted and wikilinked thoughts plus the conversation.

use super::super::consts::REFS_BTN_ID_PREFIX;
use super::super::lookup::PostLookup;
use super::handlers::OpenPostAction;
use crate::domain::{Post, PostId};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::IntoElement;
use rust_i18n::t;
use std::collections::HashSet;

/// How a thought refers to another one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReferenceKind {
    /// Embedded as a quote card.
    Quote,
    /// Linked inline with `[[post_id]]`.
    Link,
}

/// One referenced thought with a short human-readable excerpt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PostReference {
    pub id: PostId,
    pub kind: ReferenceKind,
    pub excerpt: String,
}

/// The quoted thought first, then wikilinked thoughts, without duplicates or self-links.
pub(crate) fn collect_references<L: PostLookup>(post: &Post, lookup: L) -> Vec<PostReference> {
    let quote = post.quote_id.iter().map(|id| (id, ReferenceKind::Quote));
    let links = post.linked_posts.iter().map(|id| (id, ReferenceKind::Link));
    let mut seen: HashSet<&PostId> = HashSet::from([&post.id]);
    quote
        .chain(links)
        .filter(|(id, _)| seen.insert(*id))
        .map(|(id, kind)| PostReference {
            id: id.clone(),
            kind,
            excerpt: lookup.resolve_wikilink(id.as_str()),
        })
        .collect()
}

/// Ghost "branch" button (with the reference count) opening the references menu.
pub(crate) fn render_refs_button(
    post: &Post,
    refs: Vec<PostReference>,
    on_open: OpenPostAction,
) -> impl IntoElement {
    let count = refs.len();
    let (replies, self_id) = (post.replies_count, post.id.clone());
    Button::new(format!("{REFS_BTN_ID_PREFIX}{}", post.id))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::GitBranch))
        .when(count > 0, |this| this.label(count.to_string()))
        .tooltip(t!("post.refs_tooltip").to_string())
        .dropdown_menu(move |menu, _, _| build_menu(menu, &refs, replies, &self_id, &on_open))
}

fn build_menu(
    mut menu: PopupMenu,
    refs: &[PostReference],
    replies: usize,
    self_id: &PostId,
    on_open: &OpenPostAction,
) -> PopupMenu {
    if refs.is_empty() {
        menu = menu.label(t!("post.refs_none").to_string());
    }
    for reference in refs {
        menu = menu.item(open_item(reference_label(reference), reference.id.clone(), on_open));
    }
    let thread_label = t!("post.refs_open_thread", count = replies).to_string();
    menu.separator().item(open_item(thread_label, self_id.clone(), on_open))
}

fn open_item(label: String, id: PostId, on_open: &OpenPostAction) -> PopupMenuItem {
    let on_open = on_open.clone();
    PopupMenuItem::new(label).on_click(move |_, window, cx| on_open(id.clone(), window, cx))
}

fn reference_label(reference: &PostReference) -> String {
    let excerpt = reference.excerpt.as_str();
    match reference.kind {
        ReferenceKind::Quote => t!("post.refs_quote", excerpt = excerpt),
        ReferenceKind::Link => t!("post.refs_link", excerpt = excerpt),
    }
    .to_string()
}

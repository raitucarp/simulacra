//! References dropdown of the action bar: the thoughts this one quotes, embeds (N1) and links
//! to, then the thoughts referring to it (its backlinks), and the conversation.

use super::super::consts::REFS_BTN_ID_PREFIX;
use super::super::lookup::PostLookup;
use super::handlers::OpenPostAction;
use crate::domain::post::embed_targets;
use crate::domain::post::refs::RefKind;
use crate::domain::{Post, PostId};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::IntoElement;
use crate::i18n::t_count;
use rust_i18n::t;
use std::collections::HashSet;

/// How a thought refers to another one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReferenceKind {
    /// Embedded as a quote card.
    Quote,
    /// N1: shown inside it with `((post_id))`.
    Embed,
    /// Linked inline with `[[post_id]]`.
    Link,
    /// N1: the other thought refers to this one (a backlink), this way.
    ReferredBy(RefKind),
}

/// One referenced thought with a short human-readable excerpt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PostReference {
    pub id: PostId,
    pub kind: ReferenceKind,
    pub excerpt: String,
}

/// The quoted thought first, then embedded and wikilinked thoughts, without duplicates or
/// self-links; then every thought referring to this one, newest first.
pub(crate) fn collect_references<L: PostLookup>(post: &Post, lookup: L) -> Vec<PostReference> {
    let embedded: HashSet<PostId> = embed_targets(&lookup.body_tokens(post)).into_iter().map(|(target, _)| PostId::new(target)).collect();
    let quote = post.quote_id.iter().map(|id| (id.clone(), ReferenceKind::Quote));
    let links = post.linked_posts.iter().map(|id| (id.clone(), if embedded.contains(id) { ReferenceKind::Embed } else { ReferenceKind::Link }));
    let mut seen: HashSet<PostId> = HashSet::from([post.id.clone()]);
    let outgoing: Vec<(PostId, ReferenceKind)> = quote.chain(links).filter(|(id, _)| seen.insert(id.clone())).collect();
    let incoming = lookup.backlinks(&post.id).into_iter().map(|link| (link.from, ReferenceKind::ReferredBy(link.kind)));
    outgoing.into_iter().chain(incoming).map(|(id, kind)| PostReference { excerpt: lookup.resolve_wikilink(id.as_str()), id, kind }).collect()
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
    let (incoming, outgoing): (Vec<&PostReference>, Vec<&PostReference>) = refs.iter().partition(|r| matches!(r.kind, ReferenceKind::ReferredBy(_)));
    if outgoing.is_empty() {
        menu = menu.label(t!("post.refs_none").to_string());
    }
    for reference in outgoing {
        menu = menu.item(open_item(reference_label(reference), reference.id.clone(), on_open));
    }
    if !incoming.is_empty() {
        menu = menu.separator().label(t_count("post.refs_backlinks", incoming.len()));
    }
    for reference in incoming {
        menu = menu.item(open_item(reference_label(reference), reference.id.clone(), on_open));
    }
    let thread_label = t_count("post.refs_open_thread", replies);
    menu.separator().item(open_item(thread_label, self_id.clone(), on_open))
}

fn open_item(label: String, id: PostId, on_open: &OpenPostAction) -> PopupMenuItem {
    let on_open = on_open.clone();
    PopupMenuItem::new(label).on_click(move |_, window, cx| on_open(id.clone(), window, cx))
}

/// The menu line of `reference` (also the inspector's backlink rows, N1).
pub(crate) fn reference_label(reference: &PostReference) -> String {
    let excerpt = reference.excerpt.as_str();
    match reference.kind {
        ReferenceKind::Quote => t!("post.refs_quote", excerpt = excerpt),
        ReferenceKind::Embed => t!("post.refs_embed", excerpt = excerpt),
        ReferenceKind::Link => t!("post.refs_link", excerpt = excerpt),
        ReferenceKind::ReferredBy(kind) => t!(&format!("post.refs_by.{}", kind.key()), excerpt = excerpt),
    }
    .to_string()
}

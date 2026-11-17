//! The four collapsible sections of the expanded sidebar, and the pinned shortcuts of the rail.

use super::item_row::{render_item_row, ItemRow, Leading};
use super::prefs::{NavRef, SidebarSection};
use super::resolve::{resolve_all, ResolvedRef};
use super::section::{render_section, SectionView, SECTION_PREVIEW_LIMIT};
use super::state::SidebarState;
use super::tooltip::with_tooltip;
use super::SidebarCallbacks;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Element id prefix of the rail's pinned buttons.
const RAIL_PIN_ID_PREFIX: &str = "sidebar-rail-pin-";

pub(super) fn render_sections(state: &AppState, sidebar: &SidebarState, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Vec<Div> {
    SidebarSection::ALL
        .into_iter()
        .map(|section| {
            let rows = section_rows(section, state, sidebar).into_iter().map(|row| render_item_row(row, callbacks, theme)).collect();
            let view = SectionView {
                section,
                is_collapsed: sidebar.prefs.is_collapsed(section),
                is_showing_all: sidebar.is_showing_all(section),
                rows,
            };
            render_section(view, callbacks, theme)
        })
        .collect()
}

fn row_id(kind: &str, at: usize) -> SharedString {
    SharedString::from(format!("sidebar-{kind}-{at}"))
}

/// Rows of the resolvable `references` (avatars for profiles).
fn resolved_rows(kind: &str, references: &[NavRef], state: &AppState, sidebar: &SidebarState) -> Vec<ItemRow> {
    resolve_all(state, references)
        .into_iter()
        .enumerate()
        .map(|(at, resolved): (usize, ResolvedRef)| ItemRow {
            id: row_id(kind, at),
            is_pinned: sidebar.prefs.is_pinned(&resolved.reference),
            leading: resolved.profile.map_or(Leading::Icon(resolved.icon), |profile| Leading::Avatar(Box::new(profile))),
            reference: resolved.reference,
            label: resolved.label,
            detail: None,
        })
        .collect()
}

fn section_rows(section: SidebarSection, state: &AppState, sidebar: &SidebarState) -> Vec<ItemRow> {
    let prefs = &sidebar.prefs;
    match section {
        SidebarSection::Pinned => resolved_rows("pinned", &prefs.pins, state, sidebar),
        SidebarSection::Recent => {
            let unpinned: Vec<NavRef> = prefs.recents.iter().filter(|item| !prefs.is_pinned(item)).cloned().collect();
            resolved_rows("recent", &unpinned, state, sidebar)
        }
        SidebarSection::Sources => state.current_documents().enumerate().map(|(at, doc)| {
            let reference = NavRef::Document(doc.id.clone());
            ItemRow { id: row_id("source", at), is_pinned: prefs.is_pinned(&reference), reference, label: doc.title.clone(), detail: Some(doc.chunk_count.to_string()), leading: Leading::Icon(LucideIcon::FileText) }
        }).collect(),
        SidebarSection::Members => state.current_network_agents().into_iter().enumerate().map(|(at, profile)| {
            let reference = NavRef::Profile(profile.id.clone());
            ItemRow { id: row_id("member", at), is_pinned: prefs.is_pinned(&reference), reference, label: profile.name.clone(), detail: None, leading: Leading::Avatar(Box::new(profile.clone())) }
        }).collect(),
    }
}

/// The first pins as icon buttons for the rail.
pub(super) fn render_rail_pins(state: &AppState, sidebar: &SidebarState, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Vec<Stateful<Div>> {
    resolve_all(state, &sidebar.prefs.pins).into_iter().take(SECTION_PREVIEW_LIMIT).enumerate().map(|(at, resolved)| {
        let (hover_bg, on_open, reference) = (theme.hover_bg, callbacks.on_open.clone(), resolved.reference.clone());
        let glyph = match &resolved.profile {
            Some(profile) => render_profile_avatar(profile, theme.avatar_size_sm, theme),
            None => div().text_color(theme.text_muted).child(Icon::new(resolved.icon).size(theme.icon_size_sm)),
        };
        let button = div()
            .id(SharedString::from(format!("{RAIL_PIN_ID_PREFIX}{at}")))
            .size(theme.nav_rail_button_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.radius_md)
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .on_click(move |_, window, cx| on_open(reference.clone(), window, cx))
            .child(glyph);
        with_tooltip(button, resolved.label, None)
    }).collect()
}

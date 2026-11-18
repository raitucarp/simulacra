//! A room's scene (themed SVG art) with its loci as numbered markers at the positions the
//! room kind defines: filled when something is placed, ringed when selected or current.

use crate::domain::learning::palace::rooms::ROOM_CANVAS;
use crate::domain::learning::{Room, RoomKind};
use crate::theme::ThemeTokens;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const MARKER_ID_PREFIX: &str = "learning-locus-";
const HALF: f32 = 2.0;

/// A click on locus `slot`.
pub type MarkerAction = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// The scene of `kind` at `width`.
pub fn room_art(kind: RoomKind, width: Pixels) -> Illustration {
    Illustration::asset(kind.art_path(), IllustrationKind::TraitCard).width(width)
}

/// How a marker is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerState {
    pub placed: bool,
    pub highlighted: bool,
    /// Drawn at all (a walk shows only the current locus).
    pub shown: bool,
}

fn marker(slot: usize, state: MarkerState, on_click: Option<MarkerAction>, theme: &ThemeTokens) -> Stateful<Div> {
    let half = theme.learning_marker_size / HALF;
    let (fill, text) = if state.placed { (theme.accent_dialectic, theme.bg_app) } else { (theme.bg_card, theme.text_secondary) };
    div()
        .id(ElementId::NamedInteger(MARKER_ID_PREFIX.into(), slot as u64))
        .ml(-half)
        .mt(-half)
        .size(theme.learning_marker_size)
        .rounded(theme.radius_full)
        .flex()
        .items_center()
        .justify_center()
        .bg(fill)
        .border(if state.highlighted { theme.focus_ring_width } else { theme.border_width_hairline })
        .border_color(if state.highlighted { theme.text_primary } else { theme.accent_dialectic })
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::BOLD)
        .text_color(text)
        .when_some(on_click, |m, action| m.cursor_pointer().on_click(move |_, window, cx| action(slot, window, cx)))
        .child((slot + 1).to_string())
}

/// `room`'s scene at `width` with its markers (`state_of` says how each locus is drawn).
pub fn render_scene(room: &Room, width: Pixels, state_of: &dyn Fn(usize) -> MarkerState, on_click: Option<MarkerAction>, theme: &ThemeTokens) -> Div {
    let height = width * (ROOM_CANVAS.1 / ROOM_CANVAS.0);
    let markers = room.loci.iter().filter_map(|locus| {
        let state = state_of(locus.slot);
        let (x, y) = room.kind.position(locus.slot)?;
        state.shown.then(|| div().absolute().left(relative(x)).top(relative(y)).child(marker(locus.slot, state, on_click.clone(), theme)))
    });
    div().relative().flex_none().w(width).h(height).rounded(theme.radius_lg).overflow_hidden().border(theme.border_width_hairline).border_color(theme.border_subtle).child(room_art(room.kind, width)).children(markers)
}

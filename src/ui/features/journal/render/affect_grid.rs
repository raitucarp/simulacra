//! The affect grid: a square of valence (left to right, unpleasant to pleasant) by arousal
//! (bottom to top, deactivated to activated), its quadrants softly tinted, the eight regions
//! of the circumplex labelled around it, and the chosen point. A click anywhere sets the point;
//! an invisible layer records the grid's bounds so the click can be read against them.

use super::super::panel::JournalPanel;
use super::super::text::region_label;
use crate::domain::journal::mood::{Affect, AffectRegion};
use crate::theme::color::with_alpha;
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

const GRID_ID: &str = "journal-affect-grid";
/// Half the grid, as a share.
const HALF: f32 = 0.5;

/// One quadrant at (`left`, `top`) shares, tinted in `colour` as lightly as the subtle accents.
fn quadrant(left: f32, top: f32, colour: Rgba, theme: &ThemeTokens) -> Div {
    div().absolute().left(relative(left)).top(relative(top)).w(relative(HALF)).h(relative(HALF)).bg(with_alpha(colour, theme.accent_success_subtle.a))
}

/// A zero-sized anchor at (`x`, `y`) shares that centres its child on that point.
fn anchor(x: f32, y: f32) -> Div {
    div().absolute().left(relative(x)).top(relative(y)).w(Pixels::ZERO).h(Pixels::ZERO).flex().items_center().justify_center()
}

fn region_tag(region: AffectRegion, chosen: bool, theme: &ThemeTokens) -> Div {
    let (x, y) = region.label_position();
    let colour = if chosen { theme.text_primary } else { theme.text_muted };
    let label = div().flex_none().w(theme.journal_grid_label_width).text_center().text_size(theme.font_size_2xs).text_color(colour).when(chosen, |l| l.font_weight(FontWeight::SEMIBOLD)).child(region_label(region));
    anchor(x, y).child(label)
}

fn dot(affect: Affect, theme: &ThemeTokens) -> Div {
    let (x, y) = affect.grid_position();
    let mark = div().flex_none().size(theme.journal_grid_dot_size).rounded(theme.radius_full).bg(theme.accent_brand).border(theme.border_width_threadline).border_color(theme.bg_card);
    anchor(x, y).child(mark)
}

fn bounds_probe(cell: Rc<Cell<Option<Bounds<Pixels>>>>) -> impl IntoElement {
    canvas(move |bounds, _, _| cell.set(Some(bounds)), |_, _, _, _| {}).absolute().top_0().left_0().size_full()
}

impl JournalPanel {
    pub(super) fn render_affect_grid(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let chosen = self.draft.affect.map(Affect::region);
        let axes = [
            div().absolute().left(relative(HALF)).top_0().bottom_0().w(theme.border_width_hairline).bg(theme.border),
            div().absolute().top(relative(HALF)).left_0().right_0().h(theme.border_width_hairline).bg(theme.border),
        ];
        let tags = AffectRegion::AROUND.into_iter().chain([AffectRegion::Neutral]).map(|region| region_tag(region, chosen == Some(region), theme));
        div()
            .id(GRID_ID)
            .relative()
            .flex_none()
            .size(theme.journal_grid_size)
            .rounded(theme.radius_lg)
            .overflow_hidden()
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .cursor_crosshair()
            .child(bounds_probe(Rc::clone(&self.grid_bounds)))
            .child(quadrant(0.0, 0.0, theme.accent_danger, theme))
            .child(quadrant(HALF, 0.0, theme.accent_dialectic, theme))
            .child(quadrant(0.0, HALF, theme.info, theme))
            .child(quadrant(HALF, HALF, theme.accent_success, theme))
            .children(axes)
            .children(tags)
            .children(self.draft.affect.map(|a| dot(a, theme)))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, _, cx| this.click_grid(event.position, cx)))
    }
}

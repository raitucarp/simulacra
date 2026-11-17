//! "Choose your class": eight illustrated class cards, three other ways to begin (scratch,
//! dice, describe in words), the Agent / Persona switch and a preview panel on the right.

mod classes;
mod preview;
mod side;

use super::common::{muted, title};
use crate::state::forge::ForgeKind;
use crate::theme::ThemeTokens;
use crate::ui::components::{persisted_h_resizable, segment_pill, segmented_track, Segment, SplitPane};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{KeyboardHint, KeyboardStop};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const TITLE_KEY: &str = "forge.ui.start.title";
const SUBTITLE_KEY: &str = "forge.ui.start.subtitle";
const CLOSE_KEY: &str = "forge.ui.close";
const CLOSE_ID: &str = "forge-start-close";
const GRID_SCROLL_ID: &str = "forge-start-grid";
const SIDE_SCROLL_ID: &str = "forge-start-side";
const KIND_ID_PREFIX: &str = "forge-kind";
/// Layout id of the class grid | preview split (sizes persist).
const START_SPLIT_ID: &str = "forge-start-split";

/// The class grid beside the preview panel, both resizable.
fn start_split(grid: impl IntoElement, side: impl IntoElement, theme: &ThemeTokens) -> impl IntoElement {
    persisted_h_resizable(START_SPLIT_ID, theme)
        .pane(SplitPane::flex(theme.split_flex_min).child(grid))
        .pane(SplitPane::sized(theme.forge_preview_width, theme.forge_preview_min..theme.forge_preview_max).child(side))
}

/// Icon of each kind on the switch.
fn kind_icon(kind: ForgeKind) -> LucideIcon {
    match kind {
        ForgeKind::Agent => LucideIcon::Bot,
        ForgeKind::Persona => LucideIcon::UserCheck,
    }
}

impl ForgeView {
    pub(super) fn start_screen(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let grid = div()
            .id(GRID_SCROLL_ID)
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_y_scrollbar()
            .p(theme.spacing_xl)
            .flex()
            .flex_col()
            .gap(theme.spacing_xl)
            .child(self.class_grid(handle))
            .child(self.begin_tiles(handle));
        let side = div()
            .id(SIDE_SCROLL_ID)
            .size_full()
            .overflow_y_scrollbar()
            .border_l(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .bg(theme.bg_sidebar)
            .p(theme.spacing_lg)
            .child(self.start_side_panel(handle));
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.start_header(handle))
            .child(div().flex_1().min_h_0().flex().child(start_split(grid, side, theme)))
            .into_any_element()
    }

    fn start_header(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let close = handle.act(ForgeView::close);
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.spacing_lg)
            .px(theme.spacing_xl)
            .py(theme.spacing_lg)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(div().flex().flex_col().gap(theme.spacing_2xs).child(title(tr(TITLE_KEY), theme)).child(muted(tr(SUBTITLE_KEY), theme)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing_md)
                    .child(kind_switch(self.start.kind, handle, theme))
                    .child(KeyboardHint::button(theme))
                    .child(Button::new(CLOSE_ID).ghost().icon(LucideIcon::X).tooltip(tr(CLOSE_KEY)).on_click(move |_, w, cx| close(w, cx))),
            )
    }
}

/// The Agent / Persona segmented switch.
fn kind_switch(current: ForgeKind, handle: &ForgeHandle, theme: &ThemeTokens) -> Div {
    let pills = ForgeKind::ALL.map(|kind| {
        let pick = handle.act(move |this, window, cx| this.set_kind(kind, window, cx));
        let segment = Segment {
            id: SharedString::from(format!("{KIND_ID_PREFIX}-{}", kind.key())).into(),
            icon: kind_icon(kind),
            label: tr(&kind.label_key()),
            active: kind == current,
            active_icon_color: theme.accent_brand,
        };
        let click = pick.clone();
        segment_pill(segment, theme, move |window, cx| click(window, cx)).keyboard_stop(theme, pick)
    });
    div().w(theme.forge_preview_width).child(segmented_track(theme).children(pills))
}

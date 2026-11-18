//! The Arena page: a header with its tabs, the last failure as a notice, then the current tab
//! as a setup column beside the record on show (stacked below the wide layout class).

mod claims;
mod debate_setup;
mod debate_view;
mod debaters;
mod header;
mod lab_compare;
mod lab_form;
mod lab_results;
mod lab_variables;
mod saved;
mod turns;
mod verdict;

use super::kit::caption;
use super::panel::ArenaPanel;
use super::tab::ArenaTab;
use crate::theme::ThemeTokens;
use crate::ui::components::{art_state, ArtState};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, NoticeKey};
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::Icon;
use gpui_kit::*;

const ROOT_ID: &str = "arena-root";
const SCROLL_ID: &str = "arena-scroll";

impl Render for ArenaPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let class = LayoutClass::of_window(window, &theme);
        let body = match self.tab {
            ArenaTab::Debate => self.render_debate_tab(class, cx),
            ArenaTab::Lab => self.render_lab_tab(class, cx),
        };
        let column = div()
            .w_full()
            .max_w(theme.arena_page_max_width)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_header(class, cx))
            .children(self.render_notice())
            .child(body);
        let inset = if class.is_compact() { theme.panel_padding_compact } else { theme.panel_padding };
        div()
            .id(ROOT_ID)
            .size_full()
            .track_focus(&self.focus)
            .bg(theme.bg_app)
            .font_family(theme.font_family_ui.clone())
            .child(div().id(SCROLL_ID).size_full().overflow_y_scroll().flex().justify_center().items_start().px(inset).py(theme.panel_padding).child(column))
    }
}

/// `side` beside `main`; below the wide class they stack, the open record (`main_first`)
/// above the forms so it is seen without scrolling.
pub(super) fn two_columns(class: LayoutClass, side: impl IntoElement, main: impl IntoElement, main_first: bool, theme: &ThemeTokens) -> Div {
    if class.stacks_inspector() {
        let (side, main) = (side.into_any_element(), main.into_any_element());
        let (first, second) = if main_first { (main, side) } else { (side, main) };
        return div().w_full().flex().flex_col().gap(theme.section_gap).child(first).child(second);
    }
    let side = div().w(theme.arena_side_width).flex_none().flex().flex_col().gap(theme.section_gap).child(side);
    div().w_full().flex().items_start().gap(theme.section_gap).child(side).child(div().flex_1().min_w_0().flex().flex_col().gap(theme.section_gap).child(main))
}

impl ArenaPanel {
    /// The last failure, or a missing key, as a quiet notice.
    fn render_notice(&self) -> Option<Div> {
        let theme = &self.theme;
        let text = match (&self.data.notice, self.data.has_key) {
            (Some(notice), _) => notice.clone(),
            (None, false) => super::text::tr("arena.no_key"),
            (None, true) => return None,
        };
        let row = div()
            .w_full()
            .flex()
            .items_start()
            .gap(theme.inline_gap)
            .p(theme.card_padding)
            .rounded(theme.radius_md)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(theme.warning)
            .child(Icon::new(LucideIcon::CircleAlert).size(theme.icon_size_sm).text_color(theme.warning))
            .child(caption(text, theme).flex_1().min_w_0());
        Some(row)
    }

    /// The art state shown where no record is open.
    pub(super) fn idle_state(&self, title_key: &str, hint_key: &str) -> Div {
        let theme = &self.theme;
        let art = if self.data.has_key { Illustration::spot(crate::ui::illustration::SpotKey::Debate) } else { Illustration::notice(NoticeKey::NoKey) };
        let state = ArtState::titled(art.width(theme.art_empty_width), super::text::tr(title_key), super::text::tr(hint_key));
        super::kit::surface(theme).items_center().py(theme.section_gap).child(art_state(state, theme))
    }
}

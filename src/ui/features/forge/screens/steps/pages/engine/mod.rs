//! Engine (agents): how this agent thinks out loud. Five sections behind a tab strip (one tab
//! stop, arrow keys move between them): the model it speaks through, how requests are sent,
//! the shape of its answers, its image engine and the tools it may call. Every choice is an
//! illustrated card; every section lists its own issues and opens from the Review "Fix" link.

mod credential;
mod custom_tool;
mod headers;
mod image;
mod kit;
mod model;
mod output;
mod preview;
mod probe_view;
mod request;
mod tools;

#[cfg(test)]
mod tests;

use super::super::super::common::{card, heading, muted};
use crate::state::forge::{EngineSection, Severity};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{focus_ring_shadow, CardGrid, GridCell};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use kit::engine_id;
use std::rc::Rc;

const TABS_ID: &str = "sections";

/// The icon of a section tab.
const fn section_icon(section: EngineSection) -> LucideIcon {
    match section {
        EngineSection::Model => LucideIcon::Brain,
        EngineSection::Request => LucideIcon::Send,
        EngineSection::Output => LucideIcon::Boxes,
        EngineSection::Image => LucideIcon::Image,
        EngineSection::Tools => LucideIcon::Zap,
    }
}

impl ForgeView {
    pub(in super::super) fn engine_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let section = self.engine.section;
        let body = match section {
            EngineSection::Model => self.model_section(handle),
            EngineSection::Request => self.request_section(handle),
            EngineSection::Output => self.output_section(handle),
            EngineSection::Image => self.image_section(handle),
            EngineSection::Tools => self.tools_section(handle),
        };
        let intro = card(theme).child(heading(tr(&section.label_key()), theme)).child(muted(tr(&section.intro_key()), theme));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .child(self.section_tabs(handle))
            .child(intro.children(self.issue_lines(section.issue_field())))
            .child(body)
            .into_any_element()
    }

    /// One tab per section; a dot marks sections with errors (red) or warnings (yellow).
    fn section_tabs(&self, handle: &ForgeHandle) -> CardGrid {
        let cells = EngineSection::ALL.iter().map(|&section| self.section_tab(section, handle)).collect();
        CardGrid::new(engine_id(TABS_ID), cells, EngineSection::ALL.len(), self.theme.spacing_xs)
    }

    fn section_tab(&self, section: EngineSection, handle: &ForgeHandle) -> GridCell {
        let theme = self.theme.clone();
        let selected = self.engine.section == section;
        let dot = if self.section_has(section, Severity::Error) {
            Some(theme.accent_danger)
        } else {
            self.section_has(section, Severity::Warning).then_some(theme.warning)
        };
        let build = move |focused: bool, click: crate::ui::features::forge::widgets::ActionCallback| {
            let (fg, bg) = if selected { (theme.accent_brand, theme.accent_brand_subtle) } else { (theme.text_secondary, theme.bg_card) };
            let border = if selected { theme.accent_brand } else { theme.border_subtle };
            let marker = dot.map(|color| div().absolute().top(theme.spacing_xs).right(theme.spacing_xs).size(theme.nav_active_dot_size).rounded(theme.radius_full).bg(color));
            div()
                .id(engine_id(format!("{TABS_ID}-{}", section.key())))
                .relative()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .items_center()
                .gap(theme.spacing_2xs)
                .py(theme.spacing_sm)
                .rounded(theme.radius_lg)
                .border(theme.border_width_hairline)
                .border_color(border)
                .bg(bg)
                .text_color(fg)
                .cursor_pointer()
                .hover(|style| style.bg(theme.bg_elevated))
                .when(focused, |tab| tab.shadow(focus_ring_shadow(&theme)))
                .on_click(move |_, w, cx| click(w, cx))
                .child(Icon::new(section_icon(section)).size(theme.icon_size_md))
                .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).child(tr(&format!("{}_tab", section.label_key()))))
                .children(marker)
        };
        let open = handle.act(move |this, _, cx| this.open_engine_section(section, cx));
        GridCell::new(selected, build, Rc::new(move |w, cx| open(w, cx)))
    }
}

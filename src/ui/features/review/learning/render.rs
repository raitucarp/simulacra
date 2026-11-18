//! The Learn tab: its sub-switch (Tutor, Goals, Gaps, Palace) above the chosen tool, plus
//! the small building blocks the tools share (a framed text field, a chip, a section label).

use super::super::panel::ReviewPanel;
use super::super::text::tr;
use super::state::LearnTool;
use crate::theme::ThemeTokens;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const TOOL_ID_PREFIX: &str = "learning-tool-";

/// A one-line text field in the page's input frame.
pub fn framed_input(input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div().flex_1().min_w_0().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.input_bg).px(theme.spacing_sm).child(Input::new(input).bordered(false))
}

/// A small uppercase section label.
pub fn section_label(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(text.to_uppercase())
}

/// A clickable chip, filled when `active`.
pub fn chip(id: ElementId, label: String, active: bool, theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id(id)
        .px(theme.spacing_sm)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .text_size(theme.font_size_xs)
        .cursor_pointer()
        .when(active, |c| c.bg(theme.accent_brand_subtle).border_color(theme.accent_brand_border).text_color(theme.text_primary))
        .when(!active, |c| c.border_color(theme.border).text_color(theme.text_secondary).hover(|c| c.bg(theme.hover_bg)))
        .child(label)
}

/// A bulleted line in `color`.
pub fn bullet(text: String, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .child(div().flex_none().mt(theme.spacing_xs).size(theme.review_swatch_size).rounded(theme.radius_full).bg(color))
        .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(text))
}

impl ReviewPanel {
    /// The Learn tab.
    pub(in crate::ui::features::review) fn render_learning(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        let body = match self.learning.tool {
            LearnTool::Tutor => self.render_tutor(cx),
            LearnTool::Goals => self.render_goals(class, cx),
            LearnTool::Gaps => self.render_gaps(class, cx),
            LearnTool::Palace => self.render_palace(class, cx),
        };
        div().w_full().flex().flex_col().gap(self.theme.section_gap).child(div().max_w(self.theme.review_card_max_width).child(self.render_tool_switch(cx))).child(body).into_any_element()
    }

    fn render_tool_switch(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pills = LearnTool::ALL.into_iter().map(|tool| {
            let segment = Segment {
                id: ElementId::Name(format!("{TOOL_ID_PREFIX}{}", tool.key()).into()),
                icon: tool.icon(),
                label: tr(&format!("learning.tool.{}", tool.key())),
                active: self.learning.tool == tool,
                active_icon_color: theme.accent_dialectic,
            };
            let entity = cx.entity().downgrade();
            segment_pill(segment, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.open_tool(tool, cx));
            })
        });
        segmented_track(theme).children(pills)
    }

    /// Shows `tool` in the Learn tab.
    pub fn open_tool(&mut self, tool: LearnTool, cx: &mut Context<Self>) {
        self.learning.tool = tool;
        self.show(super::super::panel::Screen::Learn, cx);
    }
}

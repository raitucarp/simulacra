//! The illustrated step rail: banner thumbnail, number and name, and a ✓, a current dot or
//! an issue count per step.

use super::step_key;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::{rail_items, RailItem, RailStatus};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::Illustration;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const RAIL_ID: &str = "forge-step-rail";
const ITEM_ID_PREFIX: &str = "forge-rail";

/// The badge at the end of a rail entry.
fn status_badge(item: &RailItem, theme: &ThemeTokens) -> Option<Div> {
    match item.status {
        RailStatus::Issues(count) => Some(
            div()
                .px(theme.spacing_xs)
                .rounded(theme.radius_full)
                .bg(theme.accent_danger)
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.on_danger)
                .child(count.to_string()),
        ),
        RailStatus::Complete => Some(div().text_color(theme.accent_success).child(Icon::new(LucideIcon::CircleCheck).size(theme.font_size_md))),
        RailStatus::Pending if item.is_current => {
            Some(div().size(theme.spacing_sm).rounded(theme.radius_full).bg(theme.accent_brand))
        }
        RailStatus::Pending => None,
    }
}

impl ForgeView {
    pub(super) fn step_rail(&self, handle: &ForgeHandle) -> impl IntoElement {
        let theme = &self.theme;
        let items: Vec<Stateful<Div>> = rail_items(&self.draft, &self.issues).iter().map(|item| self.rail_entry(item, handle)).collect();
        div()
            .id(RAIL_ID)
            .w(theme.forge_rail_width)
            .flex_shrink_0()
            .h_full()
            .overflow_y_scrollbar()
            .p(theme.spacing_sm)
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .bg(theme.bg_sidebar)
            .border_r(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .children(items)
    }

    fn rail_entry(&self, item: &RailItem, handle: &ForgeHandle) -> Stateful<Div> {
        let theme = &self.theme;
        let step = item.step;
        let go = handle.act(move |this, window, cx| this.go_to(step, window, cx));
        let click = go.clone();
        let hover_bg = theme.hover_bg;
        let label = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(item.number.to_string()))
            .child(
                div()
                    .text_size(theme.font_size_sm)
                    .font_weight(if item.is_current { FontWeight::BOLD } else { FontWeight::NORMAL })
                    .truncate()
                    .child(tr(&step.label_key())),
            );
        div()
            .id(SharedString::from(format!("{ITEM_ID_PREFIX}-{}", step.key())))
            .flex()
            .items_center()
            .gap(theme.spacing_sm)
            .p(theme.spacing_xs)
            .rounded(theme.radius_md)
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .when(item.is_current, |row| row.bg(theme.accent_brand_subtle).border(theme.border_width_hairline).border_color(theme.accent_brand_border))
            .on_click(move |_, window, cx| click(window, cx))
            .keyboard_stop(theme, go)
            .child(div().rounded(theme.radius_sm).overflow_hidden().child(Illustration::step(step_key(step)).width(theme.forge_rail_thumb_width)))
            .child(label)
            .children(status_badge(item, theme))
    }
}

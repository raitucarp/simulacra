//! The palette overlay: scrim, search row, grouped results and the key hint footer.

use super::footer::render_footer;
use super::rows::{render_group_heading, render_result_row};
use super::{CommandPalette, PaletteEvent};
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::features::command_palette::model::PaletteMode;
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::PALETTE_KEY_CONTEXT;
use gpui_kit::component::input::Input;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const BACKDROP_ID: &str = "command-palette-backdrop";
const CARD_ID: &str = "command-palette-card";
const LIST_ID: &str = "command-palette-results";

impl Render for CommandPalette {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let card = div()
            .id(CARD_ID)
            .w(theme.palette_width)
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(theme.radius_lg)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .elevation_shadow(&theme)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(self.render_search_row(&theme))
            .child(self.render_results(&theme, cx))
            .child(render_footer(&theme));
        div()
            .id(BACKDROP_ID)
            .key_context(PALETTE_KEY_CONTEXT)
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::next_group))
            .on_action(cx.listener(Self::previous_group))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::dismiss))
            .absolute()
            .inset_0()
            .occlude()
            .bg(theme.overlay)
            .flex()
            .flex_col()
            .items_center()
            .pt(theme.palette_top_offset)
            .on_mouse_down(MouseButton::Left, cx.listener(|_, _, _, cx| cx.emit(PaletteEvent::Dismiss)))
            .child(card)
    }
}

impl CommandPalette {
    fn render_search_row(&self, theme: &ThemeTokens) -> Div {
        let (icon, chip) = match self.query.mode {
            PaletteMode::Commands => (LucideIcon::Command, Some("palette.mode_commands")),
            PaletteMode::Semantic => (LucideIcon::Sparkles, Some("palette.mode_semantic")),
            PaletteMode::Everything => (LucideIcon::Search, None),
        };
        div()
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.card_padding)
            .py(theme.spacing_sm)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(div().text_color(theme.text_muted).child(Icon::new(icon)))
            .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).child(Input::new(&self.input).appearance(false)))
            .when_some(chip, |row, key| row.child(render_mode_chip(key, theme)))
    }

    fn render_results(&self, theme: &ThemeTokens, cx: &mut Context<Self>) -> AnyElement {
        if self.query.mode == PaletteMode::Semantic {
            return self.render_semantic_results(theme, cx);
        }
        if self.groups.is_empty() {
            let text = t!("palette.empty", query = self.query.text.as_str()).to_string();
            let art = crate::ui::illustration::Illustration::empty(crate::ui::illustration::EmptyKey::NoResults).width(theme.art_notice_width);
            // T16: say why nothing matched and what to try next, not only that nothing did.
            let state = crate::ui::components::art_state(crate::ui::components::ArtState::titled(art, text, t!("palette.empty_hint").to_string()), theme);
            return div().p(theme.panel_padding).flex().justify_center().child(state).into_any_element();
        }
        let mut flat = 0;
        let mut children: Vec<AnyElement> = Vec::new();
        for group in &self.groups {
            children.push(render_group_heading(group.group, theme).into_any_element());
            for ranked in &group.items {
                let item = &self.items[ranked.index];
                let index = flat;
                let row = render_result_row(index, item, &ranked.positions, index == self.selected, theme);
                children.push(row.on_click(cx.listener(move |this, _, _, cx| this.run(index, cx))).into_any_element());
                flat += 1;
            }
        }
        self.results_list(theme, children)
    }

    /// The scrolling list of result rows.
    pub(super) fn results_list(&self, theme: &ThemeTokens, children: Vec<AnyElement>) -> AnyElement {
        div()
            .id(LIST_ID)
            .max_h(theme.palette_results_max_height)
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .p(theme.spacing_xs)
            .flex()
            .flex_col()
            .children(children)
            .into_any_element()
    }
}

fn render_mode_chip(label_key: &str, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_sm)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_full)
        .bg(theme.accent_brand_subtle)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_brand_border)
        .text_size(theme.font_size_2xs)
        .text_color(theme.accent_brand_text)
        .child(t!(label_key).to_string())
}

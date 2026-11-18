//! Builders shared by the Engine sections: illustrated card grids, inputs bound to one engine
//! string, captions with explainers, placeholder chips and JSON status lines.

use super::super::super::super::common::labeled;
use crate::domain::{EngineConfig, TraitOption};
use crate::state::forge::engine::{insert_token, JsonProblem};
use crate::state::forge::EngineSection;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{tr, tr_with, trait_description, trait_label};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ForgeTextInput, IllustratedOptionGrid};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use crate::state::forge::suggest::{EngineParam, Param};
use gpui_kit::*;
use std::rc::Rc;

/// Element-id prefix of every Engine-step control.
const ID_PREFIX: &str = "forge-engine";
const CHIPS_KEY: &str = "forge.ui.engine.chips";
const JSON_OK_KEY: &str = "forge.ui.engine.json_ok";
const JSON_ERROR_KEY: &str = "forge.ui.engine.json_error";
const JSON_NOT_OBJECT_KEY: &str = "forge.ui.engine.json_not_object";
const ARG_ERROR: &str = "error";
/// Rows (min, max) of the JSON and template editors.
pub(super) const CODE_ROWS: (usize, usize) = (4, 14);
/// Rows (min, max) of short multi-line texts (descriptions, style prefixes).
pub(super) const NOTE_ROWS: (usize, usize) = (2, 5);

/// The id of the Engine-step control `name`.
pub(super) fn engine_id(name: impl std::fmt::Display) -> SharedString {
    SharedString::from(format!("{ID_PREFIX}-{name}"))
}

/// A tinted line naming the chosen option and what it does.
pub(super) fn option_line<T: TraitOption>(option: T, theme: &ThemeTokens) -> Option<Div> {
    let description = trait_description(option)?;
    Some(
        div()
            .p(theme.spacing_sm)
            .rounded(theme.radius_md)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(theme.accent_brand)
            .text_size(theme.font_size_xs)
            .line_height(relative(theme.line_height_body))
            .child(div().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(trait_label(option)))
            .child(div().text_color(theme.text_secondary).child(description)),
    )
}

/// A green / red line saying whether a JSON editor holds what it should.
pub(super) fn json_line(problem: Option<JsonProblem>, theme: &ThemeTokens) -> Div {
    let (icon, color, text) = match problem {
        None => (LucideIcon::CircleCheck, theme.accent_success, tr(JSON_OK_KEY)),
        Some(JsonProblem::NotAnObject) => (LucideIcon::CircleAlert, theme.accent_danger, tr(JSON_NOT_OBJECT_KEY)),
        Some(JsonProblem::Syntax(error)) => (LucideIcon::CircleAlert, theme.accent_danger, tr_with(JSON_ERROR_KEY, &[(ARG_ERROR, error)])),
    };
    div()
        .flex()
        .items_start()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .text_color(color)
        .child(Icon::new(icon).size(theme.font_size_sm))
        .child(div().flex_1().min_w_0().child(text))
}

impl ForgeView {
    /// Illustrated cards for `options`, ringing `selected` and badging the suggested picks of
    /// `param`; picking one runs `set` (one undo step).
    pub(super) fn engine_cards<T: TraitOption>(
        &self,
        (param, name): (EngineParam, &str),
        (selected, options): (Option<T>, &[T]),
        handle: &ForgeHandle,
        set: impl Fn(&mut EngineConfig, T) + 'static,
    ) -> IllustratedOptionGrid<T> {
        let pick = handle.engine(set);
        IllustratedOptionGrid::new(engine_id(name), selected, &self.theme, move |option, w, cx| pick(option, w, cx))
            .options(options.to_vec())
            .suggested(self.picks(Param::Engine(param)))
    }

    /// A text input over one engine string; typing in `section` merges into one undo step.
    pub(super) fn engine_input(
        &self,
        name: impl std::fmt::Display,
        value: &str,
        (section, handle): (EngineSection, &ForgeHandle),
        set: impl Fn(&mut EngineConfig, String) + 'static,
    ) -> ForgeTextInput {
        let (edit, seal) = (handle.engine_typing(section, set), handle.seal());
        ForgeTextInput::new(engine_id(name), value, &self.theme, move |text, w, cx| edit(text, w, cx)).on_blur(move |w, cx| seal(w, cx))
    }

    /// `control` under the caption of pseudo-field `path`: with "Why?", "Suggest" and AI values
    /// for a setting, plain for a group caption.
    pub(super) fn engine_field(&self, handle: &ForgeHandle, path: &str, control: impl IntoElement) -> Div {
        match Param::from_path(path) {
            Some(param) => self.hinted(param, control, handle),
            None => labeled(path, control, &self.theme),
        }
    }

    /// Clickable `{{placeholder}}` chips appending to the template `read` returns.
    pub(super) fn token_chips(
        &self,
        name: &str,
        tokens: &[&'static str],
        current: &str,
        (section, handle): (EngineSection, &ForgeHandle),
        write: impl Fn(&mut EngineConfig) -> Option<&mut String> + 'static,
    ) -> Div {
        let theme = &self.theme;
        let write = Rc::new(write);
        let chips = tokens.iter().enumerate().map(|(at, &token)| {
            let write = write.clone();
            let insert = handle.act(move |this, _, cx| {
                this.draft.seal_history();
                let write = write.clone();
                this.edit_engine(Some(section), move |engine| {
                    if let Some(text) = write(engine) {
                        *text = insert_token(text, token);
                    }
                }, cx);
                this.draft.seal_history();
            });
            let chip = Button::new(engine_id(format!("{name}-chip-{at}"))).xsmall().label(token);
            let chip = if current.contains(token) { chip.primary() } else { chip.outline() };
            chip.on_click(move |_, w, cx| insert(w, cx))
        });
        let hint = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(CHIPS_KEY));
        div().flex().flex_col().gap(theme.spacing_xs).child(hint).child(div().flex().flex_wrap().gap(theme.spacing_xs).children(chips))
    }
}

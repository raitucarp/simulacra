//! The custom prompt template: the editor, clickable `{{placeholder}}` chips that append to
//! it, and the compiled system prompt of the draft as it stands right now.

use super::super::super::super::common::{card, heading, muted};
use crate::agent::DialecticEngine;
use crate::domain::agent_config::TEMPLATE_PLACEHOLDERS;
use crate::domain::PromptAssemblyStrategy;
use crate::state::forge::{DraftEdit, TextField};
use crate::ui::features::forge::text::{tr, tr_count};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Sizable;
use gpui_kit::*;

const TEMPLATE_KEY: &str = "forge.ui.grimoire.template";
const CHIPS_KEY: &str = "forge.ui.grimoire.chips";
const UNUSED_KEY: &str = "forge.ui.grimoire.template_unused";
const COMPILED_KEY: &str = "forge.ui.grimoire.compiled";
const COMPILED_HINT_KEY: &str = "forge.ui.grimoire.compiled_hint";
const CHARS_KEY: &str = "forge.ui.grimoire.chars";
const CHIP_ID_PREFIX: &str = "forge-placeholder";
const PREVIEW_ID: &str = "forge-compiled-prompt";
const GAP: char = ' ';

/// `template` with `placeholder` appended, separated by a space unless it already ends in
/// whitespace (or is empty).
pub(super) fn insert_placeholder(template: &str, placeholder: &str) -> String {
    if template.is_empty() || template.ends_with(char::is_whitespace) {
        format!("{template}{placeholder}")
    } else {
        format!("{template}{GAP}{placeholder}")
    }
}

impl ForgeView {
    pub(super) fn template_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let custom = self.draft.profile().config.assembly_strategy == PromptAssemblyStrategy::CustomTemplate;
        card(theme)
            .child(heading(tr(TEMPLATE_KEY), theme))
            .children((!custom).then(|| muted(tr(UNUSED_KEY), theme)))
            .child(self.text_row(TextField::PromptTemplate, handle))
            .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(CHIPS_KEY)))
            .child(self.placeholder_chips(handle))
    }

    fn placeholder_chips(&self, handle: &ForgeHandle) -> Div {
        let template = &self.draft.profile().config.prompt_template;
        let chips = TEMPLATE_PLACEHOLDERS.iter().enumerate().map(|(at, &placeholder)| {
            let insert = handle.act(move |this, _, cx| {
                let next = insert_placeholder(&this.draft.profile().config.prompt_template, placeholder);
                this.draft.seal_history();
                this.apply_edit(DraftEdit::Text(TextField::PromptTemplate, next), cx);
                this.draft.seal_history();
            });
            let chip = Button::new((CHIP_ID_PREFIX, at)).xsmall().label(placeholder);
            let chip = if template.contains(placeholder) { chip.primary() } else { chip.outline() };
            chip.on_click(move |_, w, cx| insert(w, cx))
        });
        div().flex().flex_wrap().gap(self.theme.spacing_xs).children(chips)
    }

    /// The system prompt the draft would get right now (with this network's name and sources).
    pub(super) fn compiled_card(&self) -> Div {
        let theme = &self.theme;
        let prompt = DialecticEngine::system_prompt_in(&self.draft.normalized(), &self.context.prompt);
        let count = tr_count(CHARS_KEY, prompt.chars().count(), &[]);
        let body = div()
            .id(PREVIEW_ID)
            .max_h(theme.prompt_preview_height)
            .overflow_y_scrollbar()
            .p(theme.card_padding)
            .rounded(theme.radius_md)
            .bg(theme.bg_app)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .font_family(theme.font_family_mono.clone())
            .text_size(theme.font_size_2xs)
            .text_color(theme.text_secondary)
            .line_height(relative(theme.line_height_body))
            .child(prompt);
        card(theme)
            .child(div().flex().items_center().justify_between().child(heading(tr(COMPILED_KEY), theme)).child(muted(count, theme)))
            .child(muted(tr(COMPILED_HINT_KEY), theme))
            .child(body)
    }
}

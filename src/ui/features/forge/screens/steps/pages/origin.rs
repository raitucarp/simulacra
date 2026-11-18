//! Origin, part one: name (with live validation and name ideas), handle (with "suggest" and
//! availability feedback) and role title. The rest of the page is in `origin_life`.

use super::super::super::common::{card, heading, message};
use crate::domain::archetype::NameSuggestion;
use crate::domain::Palette;
use crate::state::forge::{normalize_handle, suggest_handle, DraftEdit, TextField, TraitValue};
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::component::Sizable;
use crate::state::forge::suggest::Param;
use gpui_kit::*;

const IDENTITY_KEY: &str = "forge.ui.origin.identity";
const IDEAS_KEY: &str = "forge.ui.origin.name_ideas";
const SUGGEST_KEY: &str = "forge.ui.origin.suggest_handle";
const AVAILABLE_KEY: &str = "forge.ui.origin.handle_free";
const ARG_HANDLE: &str = "handle";
const SUGGEST_ID: &str = "forge-suggest-handle";
const IDEA_ID_PREFIX: &str = "forge-name-idea";
/// Name ideas offered at once.
const MAX_IDEAS: usize = 6;

impl ForgeView {
    pub(in super::super) fn origin_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(
                card(theme)
                    .child(heading(tr(IDENTITY_KEY), theme))
                    .child(self.text_row(TextField::Name, handle))
                    .children(self.name_ideas(handle))
                    .child(self.handle_row(handle))
                    .child(self.text_row(TextField::Description, handle)),
            )
            .child(self.origin_life(handle))
            .into_any_element()
    }

    /// Curated names of the draft's palette (or of every full palette) nobody uses yet.
    fn idea_pool(&self) -> Vec<NameSuggestion> {
        let palettes: Vec<&Palette> = self.palette().map_or_else(|| Palette::full(None).collect(), |palette| vec![palette]);
        let taken = |name: &str| self.context.existing.iter().any(|p| p.name.eq_ignore_ascii_case(name));
        palettes
            .iter()
            .flat_map(|palette| palette.names.iter().cloned())
            .filter(|idea| !taken(&idea.name) && idea.name != self.draft.profile().name)
            .take(MAX_IDEAS)
            .collect()
    }

    fn name_ideas(&self, handle: &ForgeHandle) -> Option<Div> {
        let theme = &self.theme;
        let ideas = self.idea_pool();
        if ideas.is_empty() {
            return None;
        }
        let chips = ideas.into_iter().enumerate().map(|(at, idea)| {
            let label = idea.name.clone();
            let pick = handle.with(|this, idea: NameSuggestion, _, cx| {
                let edits = [
                    DraftEdit::Text(TextField::Name, idea.name),
                    DraftEdit::Trait(TraitValue::Gender(idea.gender)),
                    DraftEdit::Text(TextField::Pronouns, idea.pronouns),
                ];
                this.apply_edits(&edits, cx);
            });
            Button::new((IDEA_ID_PREFIX, at)).outline().xsmall().label(label).on_click(move |_, w, cx| pick(idea.clone(), w, cx))
        });
        Some(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(theme.spacing_xs)
                .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(IDEAS_KEY)))
                .children(chips),
        )
    }

    fn handle_row(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let path = TextField::Handle.path();
        let suggest = handle.act(|this, _, cx| {
            let suggestion = suggest_handle(&this.draft.profile().name, this.draft.id(), &this.context.existing);
            this.apply_edit(DraftEdit::Text(TextField::Handle, suggestion), cx);
        });
        let typed = self.draft.profile().handle.trim().to_string();
        let available = (!typed.is_empty() && self.field_issues(path).is_empty())
            .then(|| message(tr_with(AVAILABLE_KEY, &[(ARG_HANDLE, normalize_handle(&typed))]), theme.accent_success, theme));
        let input_row = div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().flex_1().child(self.text_input(TextField::Handle, handle)))
            .child(Button::new(SUGGEST_ID).outline().small().icon(LucideIcon::Sparkles).label(tr(SUGGEST_KEY)).on_click(move |_, w, cx| suggest(w, cx)));
        let control = self.text_with_examples(TextField::Handle, input_row, handle);
        self.hinted(Param::Text(TextField::Handle), control, handle).children(self.issue_lines(path)).children(available)
    }
}

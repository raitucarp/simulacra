//! Grimoire (agents, advanced): folded away behind "Open the grimoire". Inside: how the prompt
//! is assembled and how they think before posting, which parts of the dossier are injected,
//! the core directive, the custom template with placeholder chips, and the compiled prompt
//! of the draft. It unfolds by itself while one of its fields has an error.

mod template;

use super::super::super::common::{card, heading, muted};
use super::trait_block::explained;
use crate::state::forge::{DraftEdit, ForgeStep, Severity, TextField, ToggleField, TraitField};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ToggleCard;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::*;

const TITLE_KEY: &str = "forge.ui.grimoire.title";
const INTRO_KEY: &str = "forge.ui.grimoire.intro";
const OPEN_KEY: &str = "forge.ui.grimoire.open";
const CLOSE_KEY: &str = "forge.ui.grimoire.close";
const SPELLS_KEY: &str = "forge.ui.grimoire.spells";
const INGREDIENTS_KEY: &str = "forge.ui.grimoire.ingredients";
const DIRECTIVE_KEY: &str = "forge.ui.grimoire.directive";
const EXPANDER_ID: &str = "forge-grimoire-expander";
/// Prompt sections the injection switches control, in dossier order.
const INJECTIONS: [ToggleField; 6] = [
    ToggleField::InjectIdentityAndBio,
    ToggleField::InjectPsychologicalProfile,
    ToggleField::InjectIntelligenceProfile,
    ToggleField::InjectLinguisticStylistics,
    ToggleField::InjectSocialAndSchedule,
    ToggleField::InjectNetworkGrounding,
];

impl ForgeView {
    pub(in super::super) fn grimoire_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let page = div().flex().flex_col().gap(theme.spacing_lg).child(self.grimoire_cover(handle));
        if !self.grimoire_unfolded() {
            return page.into_any_element();
        }
        page.child(
            card(theme)
                .child(heading(tr(SPELLS_KEY), theme))
                .child(self.trait_block(TraitField::AssemblyStrategy, handle))
                .child(self.trait_block(TraitField::PrePostReasoning, handle)),
        )
        .child(self.injection_card(handle))
        .child(card(theme).child(heading(tr(DIRECTIVE_KEY), theme)).child(self.text_row(TextField::SystemPrompt, handle)))
        .child(self.template_card(handle))
        .child(self.compiled_card())
        .into_any_element()
    }

    /// Open when asked to, or while a Grimoire field has an error to fix.
    fn grimoire_unfolded(&self) -> bool {
        self.grimoire_open || self.issues.iter().any(|issue| issue.step == ForgeStep::Grimoire && issue.severity == Severity::Error)
    }

    fn grimoire_cover(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let open = self.grimoire_unfolded();
        let toggle = handle.act(move |this, _, cx| {
            this.grimoire_open = !open;
            cx.notify();
        });
        let (icon, label) = if open { (LucideIcon::ChevronDown, CLOSE_KEY) } else { (LucideIcon::BookOpen, OPEN_KEY) };
        let button = Button::new(EXPANDER_ID).outline().icon(icon).label(tr(label)).on_click(move |_, w, cx| toggle(w, cx));
        card(theme).child(heading(tr(TITLE_KEY), theme)).child(muted(tr(INTRO_KEY), theme)).child(div().flex().child(button))
    }

    fn injection_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let toggles = INJECTIONS.map(|field| {
            let edit = handle.edit(move |on: bool| DraftEdit::Toggle(field, on));
            let toggle = ToggleCard::new(field, *field.get(profile), theme, move |on, w, cx| edit(on, w, cx));
            explained(field.path(), toggle, theme)
        });
        card(theme).child(heading(tr(INGREDIENTS_KEY), theme)).children(toggles)
    }
}

#[cfg(test)]
mod tests;

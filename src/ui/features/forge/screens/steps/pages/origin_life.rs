//! Origin, part two: occupation (illustrated quick picks or free text), home, age, gender
//! (illustrated cards, keeping pronouns in step) and biography.

use super::super::super::common::{card, heading, labeled};
use super::super::pcg_spec;
use crate::domain::portrait::{AvatarMode, PortraitSpec};
use crate::domain::HumanGender;
use crate::state::forge::fields::MAX_AGE;
use crate::state::forge::{AvatarField, CountField, DraftEdit, TextField, TraitField, TraitValue};
use crate::ui::features::forge::logic::OccupationPreset;
use crate::ui::features::forge::text::{field_label, tr, tr_count};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{CardGrid, GridCell, IllustratedOptionGrid, NumberSlider, OptionCardSpec};
use crate::ui::illustration::PortraitView;
use crate::state::forge::suggest::Param;
use gpui_kit::*;

const LIFE_KEY: &str = "forge.ui.origin.life";
const PRESETS_KEY: &str = "forge.ui.origin.occupation_presets";
const AGE_FORMAT_KEY: &str = "forge.ui.origin.age_years";
const OCCUPATION_ID_PREFIX: &str = "forge-occupation";
const OCCUPATION_GRID_ID: &str = "forge-occupation-grid";
const AGE_ID: &str = "forge-age";
const GENDER_GRID_ID: &str = "forge-gender";
/// Quick-pick occupation cards per row.
const PRESET_COLUMNS: usize = 3;
/// Gender cards per row.
const GENDER_COLUMNS: usize = 4;
/// Age slider step.
const AGE_STEP: u16 = 1;

impl ForgeView {
    pub(super) fn origin_life(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        card(theme)
            .child(heading(tr(LIFE_KEY), theme))
            .child(self.occupation_presets(handle))
            .child(self.text_row(TextField::Occupation, handle))
            .child(self.text_row(TextField::LocationCulture, handle))
            .child(self.age_slider(handle))
            .child(self.gender_grid(handle))
            .child(self.text_row(TextField::Pronouns, handle))
            .child(self.text_row(TextField::Bio, handle))
    }

    fn occupation_presets(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let cells = OccupationPreset::ALL.iter().map(|&preset| self.occupation_card(preset, handle)).collect();
        let grid = CardGrid::new(OCCUPATION_GRID_ID, cells, PRESET_COLUMNS, theme.spacing_xs);
        labeled(TextField::Occupation.path(), grid, theme)
            .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(PRESETS_KEY)))
    }

    /// One quick pick: the draft's portrait (or a default one) wearing the preset's outfit.
    fn occupation_card(&self, preset: OccupationPreset, handle: &ForgeHandle) -> GridCell {
        let theme = &self.theme;
        let width = theme.forge_preset_card_width;
        let base = pcg_spec(self.draft.profile().config.avatar.as_ref()).unwrap_or_default();
        let art = PortraitView::new(AvatarMode::Pcg(PortraitSpec { outfit: preset.outfit(), ..base }), width, theme);
        let label = tr(&preset.label_key());
        let mut spec = OptionCardSpec::new((OCCUPATION_ID_PREFIX, preset as usize), art, label.clone(), width);
        spec.selected = self.draft.profile().config.occupation.trim().eq_ignore_ascii_case(&label);
        let pick = handle.act(move |this, _, cx| this.pick_occupation(preset, cx));
        GridCell::card(spec, theme, pick)
    }

    /// Writes the preset's name into Occupation and dresses a generated portrait to match.
    fn pick_occupation(&mut self, preset: OccupationPreset, cx: &mut Context<Self>) {
        let mut edits = vec![DraftEdit::Text(TextField::Occupation, tr(&preset.label_key()))];
        if let Some(spec) = pcg_spec(self.draft.profile().config.avatar.as_ref()) {
            let dressed = PortraitSpec { outfit: preset.outfit(), ..spec };
            edits.push(DraftEdit::Avatar(AvatarField::Avatar, Some(AvatarMode::Pcg(dressed))));
        }
        self.apply_edits(&edits, cx);
    }

    fn age_slider(&self, handle: &ForgeHandle) -> Div {
        let path = CountField::Age.path();
        let age = *CountField::Age.get(self.draft.profile());
        let edit = handle.edit(|age: u16| DraftEdit::Count(CountField::Age, age));
        let slider = NumberSlider::new(AGE_ID, field_label(path), age, (0, MAX_AGE, AGE_STEP), &self.theme, move |v, w, cx| edit(v, w, cx))
            .hint(self.slider_hint(Param::Count(CountField::Age), &field_label(path), handle))
            .format(|age| tr_count(AGE_FORMAT_KEY, usize::from(age), &[]));
        div().flex().flex_col().gap(self.theme.spacing_xs).child(slider).children(self.issue_lines(path))
    }

    fn gender_grid(&self, handle: &ForgeHandle) -> Div {
        let current = self.draft.profile().config.gender;
        let pick = handle.with(|this, gender: HumanGender, _, cx| this.pick_gender(gender, cx));
        let grid = IllustratedOptionGrid::new(GENDER_GRID_ID, Some(current), &self.theme, move |g, w, cx| pick(g, w, cx))
            .columns(GENDER_COLUMNS)
            .card_width(self.theme.forge_compact_card_width)
            .compact()
            .suggested(self.picks(Param::Trait(TraitField::Gender)));
        self.hinted(Param::Trait(TraitField::Gender), grid, handle)
    }

    /// Sets the gender, and the pronouns too while they still match the old default.
    fn pick_gender(&mut self, gender: HumanGender, cx: &mut Context<Self>) {
        let cfg = &self.draft.profile().config;
        let pronouns_untouched = cfg.pronouns.trim().is_empty() || cfg.pronouns == cfg.gender.default_pronouns();
        let mut edits = vec![DraftEdit::Trait(TraitValue::Gender(gender))];
        if pronouns_untouched {
            edits.push(DraftEdit::Text(TextField::Pronouns, gender.default_pronouns().to_string()));
        }
        self.apply_edits(&edits, cx);
    }
}

//! The active-hours caption with its "?", "Why?" and "Suggest", and the typical start and end
//! for the chosen chronotype under the hour picker (sb D1).

use crate::state::forge::suggest::{range_hint, Param, SuggestContext};
use crate::state::forge::HourField;
use crate::ui::features::forge::screens::common::message;
use crate::ui::features::forge::text::{field_label, tr};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::suggest::band_caption;
use crate::ui::features::forge::widgets::ExplainerButton;
use gpui_kit::*;

const HOURS_KEY: &str = "forge.ui.daily.hours";
const LABEL_SEPARATOR: &str = ": ";

impl ForgeView {
    /// "Active hours" with the hour fields' explainer and the start's hint actions.
    pub(super) fn hours_caption(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(tr(HOURS_KEY)))
            .children(ExplainerButton::for_field(HourField::ActiveHoursStart.path(), theme))
            .child(div().flex_1())
            .child(self.hint_actions(Param::Hour(HourField::ActiveHoursStart), handle))
    }

    /// The typical start and end, then any AI values for either.
    pub(super) fn hours_hints(&self, handle: &ForgeHandle) -> Vec<AnyElement> {
        let theme = &self.theme;
        let ctx = SuggestContext::of(&self.draft);
        HourField::ALL
            .iter()
            .flat_map(|field| {
                let param = Param::Hour(*field);
                let line = range_hint(param, &ctx)
                    .map(|hint| message(format!("{}{LABEL_SEPARATOR}{}", field_label(field.path()), band_caption(&hint)), theme.text_muted, theme).into_any_element());
                line.into_iter().chain(self.ai_chips(param, handle).map(IntoElement::into_any_element))
            })
            .collect()
    }
}

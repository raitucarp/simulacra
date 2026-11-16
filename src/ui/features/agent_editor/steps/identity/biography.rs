//! Sub-tab "Biography & Lifestyle": bio, hobbies, routine and core values.

use crate::ui::components::{
    card_stack, fill, labeled_input, labeled_textarea, section_card, split_row,
};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "BIOGRAPHY, HOBBIES & VALUES";
const SUBTITLE: &str = "Lived history, personal passions, moral convictions, and daily rhythm.";
const BIO_LABEL: &str = "Personal Biography (Lived History)";
const HOBBIES_LABEL: &str = "Hobbies, Interests & Passions";
const ROUTINE_LABEL: &str = "Daily Routine & Habits";
const VALUES_LABEL: &str = "Core Moral & Intellectual Values";
const SAVE_BUTTON_ID: &str = "save-identity-sub2-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, inputs) = (ctx.theme, ctx.inputs);
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(labeled_textarea(BIO_LABEL, inputs.bio_input, theme))
            .child(
                split_row(theme.spacing_md)
                    .child(fill(labeled_input(HOBBIES_LABEL, inputs.hobbies_input, theme)))
                    .child(fill(labeled_input(ROUTINE_LABEL, inputs.routine_input, theme))),
            )
            .child(labeled_input(VALUES_LABEL, inputs.values_input, theme))
            .child(ctx.save_footer(SAVE_BUTTON_ID)),
    )
}

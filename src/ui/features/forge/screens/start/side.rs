//! The right-hand panel of the start screen: Mnemo's welcome, a class preview, the dice
//! result or the describe-in-words box.

use super::super::common::{heading, message, muted};
use super::super::steps::{class_badge, stage_portrait};
use crate::domain::ArchetypeClass;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{DescribeStatus, ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ForgeTextInput;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Disableable;
use gpui_kit::*;

const HINT_KEY: &str = "forge.ui.start.hover_hint";
const ROLLED_KEY: &str = "forge.ui.start.rolled_as";
const REROLL_KEY: &str = "forge.ui.start.reroll";
const KEEP_KEY: &str = "forge.ui.start.keep_roll";
const DESCRIBE_TITLE: &str = "forge.ui.start.describe_title";
const DESCRIBE_PLACEHOLDER: &str = "forge.ui.start.describe_placeholder";
const DESCRIBE_GO: &str = "forge.ui.start.describe_go";
const ARG_CLASS: &str = "class";
const REROLL_ID: &str = "forge-reroll";
const KEEP_ID: &str = "forge-keep-roll";
const DESCRIBE_INPUT_ID: &str = "forge-describe-text";
const DESCRIBE_GO_ID: &str = "forge-describe-go";
/// Rows of the description box (min, max).
const DESCRIBE_ROWS: (usize, usize) = (5, 10);

impl ForgeView {
    pub(super) fn start_side_panel(&self, handle: &ForgeHandle) -> AnyElement {
        if self.start.describing {
            return self.describe_panel(handle).into_any_element();
        }
        if let Some(class) = self.start.rolled {
            return self.roll_panel(class, handle).into_any_element();
        }
        if let Some(class) = self.start.preview {
            return self.class_preview(class, handle).into_any_element();
        }
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .children(self.guide_panel(handle))
            .child(muted(tr(HINT_KEY), theme))
            .into_any_element()
    }

    fn roll_panel(&self, class: ArchetypeClass, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let (reroll, keep) = (handle.act(ForgeView::roll), handle.act(ForgeView::keep_roll));
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.spacing_md)
            .child(muted(tr_with(ROLLED_KEY, &[(ARG_CLASS, tr(&class.label_key()))]), theme))
            .child(stage_portrait(profile, theme.forge_portrait_size, theme))
            .child(heading(profile.name.clone(), theme))
            .child(muted(profile.handle.clone(), theme))
            .child(class_badge(class, theme))
            .child(
                div()
                    .flex()
                    .gap(theme.spacing_sm)
                    .child(Button::new(REROLL_ID).outline().icon(LucideIcon::Dices).label(tr(REROLL_KEY)).on_click(move |_, w, cx| reroll(w, cx)))
                    .child(Button::new(KEEP_ID).primary().icon(LucideIcon::Check).label(tr(KEEP_KEY)).on_click(move |_, w, cx| keep(w, cx))),
            )
    }

    fn describe_panel(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let waiting = self.start.describe_status == DescribeStatus::Waiting;
        let go = handle.act(ForgeView::describe);
        let input = ForgeTextInput::new(DESCRIBE_INPUT_ID, self.start.describe_text.clone(), theme, {
            let set = handle.with(ForgeView::set_describe_text);
            move |text, window, cx| set(text, window, cx)
        })
        .multiline(DESCRIBE_ROWS.0, DESCRIBE_ROWS.1)
        .placeholder(tr(DESCRIBE_PLACEHOLDER));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .children(self.guide_panel(handle))
            .child(heading(tr(DESCRIBE_TITLE), theme))
            .child(input)
            .children(self.describe_message().map(|text| message(text, theme.warning, theme)))
            .child(
                Button::new(DESCRIBE_GO_ID)
                    .primary()
                    .icon(LucideIcon::Sparkles)
                    .label(tr(DESCRIBE_GO))
                    .loading(waiting)
                    .disabled(waiting)
                    .on_click(move |_, w, cx| go(w, cx)),
            )
    }
}

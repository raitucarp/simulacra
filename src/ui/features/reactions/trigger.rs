//! The card's "Add a reaction" button, which opens the picker in a popover.

use super::picker::picker_content;
use super::{ReactionControls, ReactionEvent};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const BUTTON_ID_PREFIX: &str = "reaction-add-";
const POPOVER_ID_PREFIX: &str = "reaction-picker-";

/// The button and, while it is open on this card, the picker.
pub fn render_add_reaction(controls: &ReactionControls, theme: &ThemeTokens) -> AnyElement {
    let post = &controls.post_id;
    let trigger = Button::new(SharedString::from(format!("{BUTTON_ID_PREFIX}{post}")))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::FaceSlightlySmilingPlus).text_color(theme.text_muted))
        .tooltip(t!("reactions.add").to_string());
    let (on_event, post_id, content, content_theme) = (controls.on_event.clone(), post.clone(), controls.clone(), theme.clone());
    Popover::new(SharedString::from(format!("{POPOVER_ID_PREFIX}{post}")))
        .anchor(Anchor::TopLeft)
        .trigger(trigger)
        .open(controls.picker.is_some())
        .on_open_change(move |open, window, cx| {
            let event = if *open { ReactionEvent::OpenPicker(post_id.clone()) } else { ReactionEvent::ClosePicker };
            on_event(event, window, cx)
        })
        .content(move |_, _, cx| match content.picker.as_ref() {
            Some(picker) => {
                let query = picker.search.read(cx).value().to_string();
                picker_content(&content, picker, &query, &content_theme).into_any_element()
            }
            None => div().into_any_element(),
        })
        .into_any_element()
}

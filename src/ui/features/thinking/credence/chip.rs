//! The chip itself: the percentage, coloured by where the prediction stands (open, due,
//! true, false, ambiguous), as the trigger of the popover; and the "…" menu item that opens
//! the popover on a post without a credence yet.

use super::super::labels::{outcome_colour, tr};
use super::popover::{popover_content, summary};
use super::{CredenceControls, CredenceEvent};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const CHIP_ID_PREFIX: &str = "credence-chip-";
const POPOVER_ID_PREFIX: &str = "credence-popover-";

/// The chip, or `None` when the post has no credence and its popover is closed.
pub fn render_credence_chip(controls: &CredenceControls, theme: &ThemeTokens) -> Option<AnyElement> {
    if controls.chip.is_none() && !controls.open {
        return None;
    }
    let chip = controls.chip.clone();
    let due = chip.as_ref().is_some_and(|c| c.is_due);
    let colour = match &chip {
        Some(_) if due => theme.warning,
        Some(chip) => outcome_colour(chip.outcome, theme),
        None => theme.text_muted,
    };
    let label = chip.as_ref().map_or_else(|| tr("thinking.credence.title"), |chip| format!("{}%", chip.percent));
    let tooltip = chip.as_ref().map_or_else(|| tr("thinking.credence.title"), summary);
    let post = &controls.post_id;
    let trigger = Button::new(SharedString::from(format!("{CHIP_ID_PREFIX}{post}")))
        .ghost()
        .xsmall()
        .icon(Icon::new(if due { LucideIcon::CalendarClock } else { LucideIcon::Percent }).text_color(colour))
        .label(label)
        .tooltip(tooltip);
    let (on_event, post_id, content) = (controls.on_event.clone(), post.clone(), controls.clone());
    let content_theme = theme.clone();
    let popover = Popover::new(SharedString::from(format!("{POPOVER_ID_PREFIX}{post}")))
        .anchor(Anchor::TopRight)
        .trigger(trigger)
        .open(controls.open)
        .on_open_change(move |open, window, cx| {
            let event = if *open { CredenceEvent::Open(post_id.clone()) } else { CredenceEvent::Close };
            on_event(event, window, cx)
        })
        .content(move |_, _, _| popover_content(&content.post_id, &content, &content_theme));
    Some(popover.into_any_element())
}

/// `menu` with "Set a credence…" when the post is the user's and has none yet.
pub fn with_credence_item(menu: PopupMenu, controls: Option<&CredenceControls>) -> PopupMenu {
    let Some(controls) = controls.filter(|c| c.chip.is_none()) else { return menu };
    let (on_event, post) = (controls.on_event.clone(), controls.post_id.clone());
    let item = PopupMenuItem::new(tr("thinking.credence.set")).icon(Icon::new(LucideIcon::Percent)).on_click(move |_, window, cx| on_event(CredenceEvent::Open(post.clone()), window, cx));
    menu.item(item)
}

//! A pack's picker card: three samples drawn through the pack, its name (ticked when in
//! use), description and coverage; installed packs add Remove, confirmed in place.

use super::super::common::{field_title, muted_note};
use super::{IllustrationPacksProps, PackAction, PackCardView};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::Illustration;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const CARD_ID: &str = "settings-pack-card-";
const REMOVE_ID: &str = "settings-pack-remove-";
const CONFIRM_ID: &str = "settings-pack-confirm-";
const KEEP_ID: &str = "settings-pack-keep-";

/// The card of `view`.
pub(super) fn render(theme: &ThemeTokens, view: &PackCardView, props: &IllustrationPacksProps) -> Div {
    let hover_border = theme.border_strong;
    let select = button_action(props, PackAction::Select(view.id.clone()));
    let picker = div()
        .id(SharedString::from(format!("{CARD_ID}{}", view.id)))
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .cursor_pointer()
        .on_click(move |_, window, cx| select(window, cx))
        .child(samples(theme, view))
        .child(title_row(theme, view))
        .child(muted_note(theme, view.description.clone()))
        .child(muted_note(theme, t!("packs.coverage", covered = view.covered, total = view.total).to_string()));
    div()
        .w(theme.pack_card_width)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .p(theme.spacing_sm)
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(if view.active { theme.border_width_threadline } else { theme.border_width_hairline })
        .border_color(if view.active { theme.accent_brand } else { theme.border })
        .when(!view.active, |card| card.hover(move |s| s.border_color(hover_border)))
        .child(picker)
        .when(view.removable, |card| card.child(remove_row(theme, view, props)))
}

/// The three samples, each drawn through this pack (Classic where it has none).
fn samples(theme: &ThemeTokens, view: &PackCardView) -> Div {
    let art = view.samples.iter().map(|(path, kind)| Illustration::asset(path.clone(), *kind).pack(view.id.clone()).height(theme.pack_sample_height));
    div().flex().items_center().justify_center().gap(theme.spacing_xs).py(theme.spacing_2xs).rounded(theme.radius_md).bg(theme.bg_card).children(art)
}

fn title_row(theme: &ThemeTokens, view: &PackCardView) -> Div {
    let tick = div()
        .flex()
        .items_center()
        .gap(theme.spacing_3xs)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.accent_brand_text)
        .child(Icon::new(LucideIcon::Check).size(theme.icon_size_sm))
        .child(t!("packs.active").to_string());
    div().flex().items_center().justify_between().gap(theme.spacing_xs).child(field_title(theme, view.name.clone())).when(view.active, |row| row.child(tick))
}

/// Remove, or the in-place confirmation while it is pending.
fn remove_row(theme: &ThemeTokens, view: &PackCardView, props: &IllustrationPacksProps) -> Div {
    let id = &view.id;
    let button = |prefix: &str, label: String, action: PackAction| {
        let run = button_action(props, action);
        Button::new(SharedString::from(format!("{prefix}{id}"))).ghost().xsmall().label(label).on_click(move |_, window, cx| run(window, cx))
    };
    let row = div().flex().items_center().justify_end().gap(theme.spacing_xs);
    if props.confirm_remove.as_deref() == Some(id.as_str()) {
        return row
            .child(div().flex_1().text_size(theme.font_size_2xs).text_color(theme.text_secondary).child(t!("packs.remove_confirm").to_string()))
            .child(button(KEEP_ID, t!("packs.keep").to_string(), PackAction::CancelRemove))
            .child(button(CONFIRM_ID, t!("packs.remove_yes").to_string(), PackAction::ConfirmRemove(id.clone())).danger());
    }
    row.child(button(REMOVE_ID, t!("packs.remove").to_string(), PackAction::AskRemove(id.clone())).icon(LucideIcon::Trash))
}

/// A `(window, app)` callback raising `action`.
pub(super) fn button_action(props: &IllustrationPacksProps, action: PackAction) -> impl Fn(&mut Window, &mut App) + 'static {
    let on_action = props.on_action.clone();
    move |window, cx| on_action(action.clone(), window, cx)
}

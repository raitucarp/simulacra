//! L11: the card of an attached file that is no longer where the post says (moved, renamed
//! or deleted): its art dimmed, its name struck through, why, and "Relink…" to pick where it
//! is now; when a file with the same contents sits in the old folder (it was renamed), a
//! button relinks to it at once.

use super::consts::{FILE_FOUND_ID, FILE_RELINK_ID};
use super::file_card::{card_frame, FileHost};
use super::RelinkFile;
use crate::domain::media::files::file_family;
use crate::domain::media::FileRef;
use crate::theme::ThemeTokens;
use crate::ui::features::import::family_art;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, AnyElement, Div, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled};
use rust_i18n::t;

/// The card of the `index`-th file of a post, which is missing; `found` is a renamed copy.
pub(super) fn missing_file(index: usize, file: &FileRef, found: Option<&str>, host: &FileHost<'_>, theme: &ThemeTokens) -> AnyElement {
    let art = Illustration::asset(family_art(file_family(&file.mime)), IllustrationKind::TraitCard).width(theme.file_card_art_width);
    let text = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).line_through().truncate().child(file.name().to_string()))
        .child(div().text_size(theme.font_size_xs).text_color(theme.warning).child(t!("media.file.missing").to_string()))
        .child(relink_actions(index, found, host, theme));
    card_frame(index, theme).child(div().flex_none().opacity(theme.file_missing_opacity).child(art)).child(text).into_any_element()
}

/// "Relink…" and, when a renamed copy was found, "Use <name>".
fn relink_actions(index: usize, found: Option<&str>, host: &FileHost<'_>, theme: &ThemeTokens) -> Div {
    let row = div().flex().flex_wrap().items_center().gap(theme.control_gap).on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());
    let Some(on_relink) = host.on_relink.cloned() else { return row };
    let request = RelinkFile { post_id: host.post_id.clone(), index, to: None };
    let pick = {
        let (on_relink, request) = (on_relink.clone(), request.clone());
        Button::new((FILE_RELINK_ID, index)).ghost().xsmall().icon(Icon::new(LucideIcon::Search)).label(t!("media.file.relink").to_string()).on_click(move |_, window, cx| on_relink(request.clone(), window, cx))
    };
    let renamed = found.map(|path| {
        let name = FileRef::at(path).name().to_string();
        let request = RelinkFile { to: Some(path.to_string()), ..request.clone() };
        Button::new((FILE_FOUND_ID, index)).ghost().xsmall().icon(Icon::new(LucideIcon::Link2)).label(t!("media.file.relink_to", name = name).to_string()).on_click(move |_, window, cx| on_relink(request.clone(), window, cx))
    });
    row.child(pick).children(renamed)
}

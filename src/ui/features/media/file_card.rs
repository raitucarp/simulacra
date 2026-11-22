//! L4: the card of a local file attached to a post: its type art, name, size, date and type,
//! with "Open" (the app the system uses for it) and "Show in folder". The file manager does
//! both (`crate::media::files`); the card never reads the file itself.

use super::consts::{FILE_CARD_ID, FILE_OPEN_ID, FILE_REVEAL_ID, META_SEPARATOR};
use crate::domain::media::files::file_family;
use super::OpenMediaAction;
use crate::domain::media::{FileRef, MediaCatalog};
use crate::import::FormatFamily;
use crate::theme::ThemeTokens;
use crate::ui::features::import::{family_art, family_label, file_size};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::{Icon, Sizable, WindowExt};
use gpui_kit::{div, AnyElement, App, Div, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement, SharedString, Stateful, Styled, Window};
use rust_i18n::t;
use std::path::PathBuf;

/// How a file's date is written on its card.
const DATE_FORMAT: &str = "%Y-%m-%d";

/// The card the `index`-th file of a post shows: an image is previewed (L5), a video gets
/// its poster and length (L7), anything else the plain file card.
pub(super) fn render_file(index: usize, file: &FileRef, catalog: Option<&MediaCatalog>, on_media: Option<&OpenMediaAction>, theme: &ThemeTokens) -> AnyElement {
    match file_family(&file.mime) {
        FormatFamily::Image => super::file_image::image_file(index, file, catalog, on_media, theme),
        FormatFamily::Video => super::file_video::video_file(index, file, catalog, theme),
        _ => file_card(index, file, theme).into_any_element(),
    }
}

/// The card of `file`, the `index`-th file of its post.
pub(super) fn file_card(index: usize, file: &FileRef, theme: &ThemeTokens) -> Stateful<Div> {
    file_card_with(index, file, file_family(&file.mime), None, theme)
}

/// The file card of `file` as `family`, its facts line ending in `detail` (a length).
pub(super) fn file_card_with(index: usize, file: &FileRef, family: FormatFamily, detail: Option<String>, theme: &ThemeTokens) -> Stateful<Div> {
    let art = Illustration::asset(family_art(family), IllustrationKind::TraitCard).width(theme.file_card_art_width);
    card_frame(index, theme).child(div().flex_none().child(art)).child(file_text(index, file, family, detail, theme))
}

/// The name, the facts line (ending in `detail`) and the buttons of a file.
pub(super) fn file_text(index: usize, file: &FileRef, family: FormatFamily, detail: Option<String>, theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).truncate().child(file.name().to_string()))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).truncate().child(file_meta(file, &family_label(family), detail)))
        .child(file_actions(index, &file.path, theme))
}

/// The bordered surface every file card shares.
pub(super) fn card_frame(index: usize, theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id((FILE_CARD_ID, index))
        .w_full()
        .min_w_0()
        .p(theme.card_padding)
        .flex()
        .items_center()
        .gap(theme.stack_gap)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
}

/// `PDF · 1.2 MB · 2026-11-02`, then `detail` when there is one.
pub(super) fn file_meta(file: &FileRef, kind: &str, detail: Option<String>) -> String {
    let date = file.modified.map(|at| crate::time::local(at).strftime(DATE_FORMAT).to_string());
    [Some(kind.to_string()), Some(file_size(file.size)), date, detail].into_iter().flatten().collect::<Vec<_>>().join(META_SEPARATOR)
}

/// "Open" and "Show in folder"; pressing them never selects the post.
pub(super) fn file_actions(index: usize, path: &str, theme: &ThemeTokens) -> Div {
    let (open_path, reveal_path) = (PathBuf::from(path), PathBuf::from(path));
    let open = Button::new((FILE_OPEN_ID, index)).ghost().xsmall().icon(Icon::new(LucideIcon::ExternalLink)).label(t!("media.file.open").to_string())
        .on_click(move |_, window, cx| report(crate::media::files::open_file(&open_path), window, cx));
    let reveal = Button::new((FILE_REVEAL_ID, index)).ghost().xsmall().icon(Icon::new(LucideIcon::Folder)).label(t!("media.file.show_in_folder").to_string())
        .on_click(move |_, window, cx| report(crate::media::files::show_in_folder(&reveal_path), window, cx));
    div().flex().flex_wrap().items_center().gap(theme.control_gap).on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()).child(open).child(reveal)
}

/// Tells the user when the system could not open the file.
pub(super) fn report(result: Result<(), String>, window: &mut Window, cx: &mut App) {
    if let Err(reason) = result {
        let text: SharedString = t!("media.file.open_failed", reason = reason).to_string().into();
        window.push_notification(Notification::error(text), cx);
    }
}

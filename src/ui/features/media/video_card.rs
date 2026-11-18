//! Video links: a 16:9 frame (the still when one was fetched) with a play badge and the
//! duration, then the title and "Video · domain". It opens in the system browser.

use super::consts::{META_SEPARATOR, VIDEO_CARD_ID};
use super::image_tile::round_corners;
use super::link_card::{opens_url, title_text};
use crate::domain::media::video::VIDEO_ASPECT;
use crate::domain::media::{Corners, display_domain, format_duration, link_file_name, recognise_video, PreviewEntry, VideoSource};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, img, Div, FontWeight, ObjectFit, ParentElement, Stateful, Styled, StyledImage};
use rust_i18n::t;
use std::path::PathBuf;

/// The corners of a card its top frame touches.
const TOP_CORNERS: Corners = [true, true, false, false];

/// The card of a video link (`entry` is the fetched preview, when there is one; a video
/// file is titled by its file name).
pub(super) fn video_card(url: &str, entry: Option<&PreviewEntry>, theme: &ThemeTokens) -> Stateful<Div> {
    let mut preview = entry.and_then(|entry| entry.preview.clone()).unwrap_or_default();
    let is_file = recognise_video(url).is_some_and(|video| video.source == VideoSource::File);
    if preview.title.is_empty() && is_file {
        preview.title = link_file_name(url).unwrap_or_default();
    }
    let still = entry.and_then(|entry| entry.image_path.clone());
    let meta = format!("{}{META_SEPARATOR}{}", t!("media.video"), display_domain(url));
    let caption = div()
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .when(!preview.title.is_empty(), |col| col.child(title_text(&preview.title, theme)))
        .child(div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_xs).text_color(theme.text_muted).child(Icon::new(LucideIcon::Play).size(theme.icon_size_xs)).child(meta));
    opens_url(VIDEO_CARD_ID, url, theme).flex().flex_col().child(frame(still, preview.duration_secs, theme)).child(caption)
}

/// The 16:9 still (or an empty frame) with the play badge and the duration pill.
fn frame(still: Option<String>, duration: Option<u32>, theme: &ThemeTokens) -> Div {
    let badge = div()
        .size(theme.video_play_badge_size)
        .rounded(theme.radius_full)
        .bg(theme.bg_scrim)
        .text_color(theme.on_overlay)
        .flex()
        .items_center()
        .justify_center()
        .child(Icon::new(LucideIcon::Play).size(theme.icon_size_2xl));
    let pill = duration.map(|seconds| {
        div()
            .absolute()
            .right(theme.spacing_sm)
            .bottom(theme.spacing_sm)
            .px(theme.spacing_xs)
            .rounded(theme.radius_sm)
            .bg(theme.bg_scrim)
            .text_color(theme.on_overlay)
            .text_size(theme.font_size_2xs)
            .font_weight(FontWeight::SEMIBOLD)
            .child(format_duration(seconds))
    });
    div()
        .relative()
        .w_full()
        .aspect_ratio(VIDEO_ASPECT)
        .overflow_hidden()
        .bg(theme.bg_elevated)
        .children(still.map(|path| round_corners(img(PathBuf::from(path)), TOP_CORNERS, theme.radius_lg).absolute().inset_0().size_full().object_fit(ObjectFit::Cover)))
        .child(div().absolute().inset_0().flex().items_center().justify_center().child(badge))
        .children(pill)
}

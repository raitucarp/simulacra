//! What a post shows under its body: its images (one, or a gallery), a card for each attached
//! file (L4), and a card for its first link: a video card, a link preview card, a loading
//! skeleton or a plain chip.

use super::file_card::render_file;
use super::gallery::render_gallery;
use super::link_card::{link_card, link_chip, link_skeleton};
use super::video_card::video_card;
use super::OpenMediaAction;
use crate::domain::media::{post_images, post_links, recognise_video, PreviewLookup, VideoSource};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::post::PostLookup;
use gpui_kit::{div, AnyElement, Div, IntoElement, ParentElement, Styled};

/// The media block of `post`, or `None` when it has no images, files or links.
pub fn render_post_media<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, on_media: Option<&OpenMediaAction>) -> Option<Div> {
    let images = post_images(post);
    let gallery = (!images.is_empty()).then(|| render_gallery(images, lookup.media_catalog(), theme, on_media.cloned()));
    let files: Vec<AnyElement> = post.extras.files.iter().enumerate().map(|(index, file)| render_file(index, file, lookup.media_catalog(), on_media, theme)).collect();
    let link = post_links(post.link_url.as_deref(), &post.content).into_iter().next().map(|url| render_link(&url, lookup, theme));
    if gallery.is_none() && files.is_empty() && link.is_none() {
        return None;
    }
    Some(div().w_full().min_w_0().flex().flex_col().items_start().gap(theme.stack_gap).children(gallery).children(files).children(link))
}

/// The card of one link, from what the cache holds right now.
fn render_link<L: PostLookup>(url: &str, lookup: L, theme: &ThemeTokens) -> AnyElement {
    let video = recognise_video(url);
    if video.as_ref().is_some_and(|video| video.source == VideoSource::File) {
        return video_card(url, None, theme).into_any_element();
    }
    if !lookup.link_previews_enabled() {
        return link_chip(url, video.is_some(), theme).into_any_element();
    }
    let cached = lookup.media_catalog().map_or(PreviewLookup::Unavailable, |catalog| catalog.preview(url));
    match cached {
        PreviewLookup::Ready(entry) if video.is_some() || entry.preview.as_ref().is_some_and(|p| p.is_video) => {
            video_card(url, Some(entry), theme).into_any_element()
        }
        PreviewLookup::Ready(entry) => link_card(url, entry, theme).into_any_element(),
        PreviewLookup::Loading if video.is_some() => video_card(url, None, theme).into_any_element(),
        PreviewLookup::Loading => link_skeleton(theme).into_any_element(),
        PreviewLookup::Unavailable => link_chip(url, video.is_some(), theme).into_any_element(),
    }
}

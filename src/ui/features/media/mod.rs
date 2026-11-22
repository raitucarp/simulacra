//! Post media rendering (sb C3): image galleries, link preview cards, video cards, plain link
//! chips and the lightbox. Cards only read what the background work cached; they never wait
//! on the network or decode on the UI thread (thumbnails come from the media scan).
//!
//! - `post_media`: what a post shows under its body (gallery, file cards, then its first link).
//! - `file_card`: a local file attached to the post (L4): type art, name, facts, Open, Show in folder.
//! - `file_image`: an image file previewed through the image pipeline (L5).
//! - `file_video`: a video file with its OS poster frame and length (L7), else the file card.
//! - `gallery` / `image_tile`: one image at its aspect ratio, or a 2–4 tile grid.
//! - `link_card` / `video_card`: preview cards, the loading skeleton and the fallback chip.
//! - `lightbox`: the full-window viewer (Esc closes, arrows move, images fit the window).

mod consts;
mod file_card;
mod file_image;
mod file_video;
mod gallery;
mod image_tile;
mod lightbox;
mod link_card;
mod post_media;
mod video_card;

pub use lightbox::{CloseLightbox, Lightbox, LightboxActions, NextImage, PreviousImage};
pub use post_media::render_post_media;

use crate::domain::media::ImageAttachment;
use gpui_kit::{App, Window};
use std::rc::Rc;

/// The images of one post and the one to show first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GalleryOpen {
    pub images: Rc<[ImageAttachment]>,
    pub index: usize,
}

/// Opens the lightbox on an image of a post.
pub type OpenMediaAction = Rc<dyn Fn(GalleryOpen, &mut Window, &mut App) + 'static>;

#[cfg(test)]
mod tests;

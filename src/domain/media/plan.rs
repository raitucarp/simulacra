//! What the shown posts need from the background: the links whose previews they show and
//! the images they draw (attached images and image files). Pure; the shell feeds it the posts on screen.

use super::files::{file_family, FileRef};
use super::image::post_images;
use super::links::post_links;
use super::video::{recognise_video, VideoSource};
use crate::domain::Post;
use crate::import::FormatFamily;

/// Links to preview (with whether a private address is allowed), images and files to scan.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MediaWants {
    pub links: Vec<WantedLink>,
    pub images: Vec<String>,
    /// The local files attached to the posts (L7), one per path.
    pub files: Vec<FileRef>,
}

/// A link to preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WantedLink {
    pub url: String,
    /// Only links the user wrote may point at loopback or private addresses; links in agent
    /// posts never make the app probe the local network.
    pub allow_private: bool,
}

/// The wants of `posts`, each paired with whether the user wrote it. With previews off no
/// link is fetched; links straight to a video file are never fetched (there is no page).
pub fn media_wants<'a>(posts: impl IntoIterator<Item = (&'a Post, bool)>, previews_enabled: bool) -> MediaWants {
    let mut wants = MediaWants::default();
    for (post, by_user) in posts {
        for path in post_images(post).into_iter().map(|image| image.path).chain(image_files(post)) {
            if !wants.images.contains(&path) {
                wants.images.push(path);
            }
        }
        for file in &post.extras.files {
            if !wants.files.iter().any(|known| known.path == file.path) {
                wants.files.push(file.clone());
            }
        }
        let first_link = post_links(post.link_url.as_deref(), &post.content).into_iter().next();
        let Some(url) = first_link.filter(|_| previews_enabled) else { continue };
        if recognise_video(&url).is_some_and(|video| video.source == VideoSource::File) {
            continue;
        }
        match wants.links.iter_mut().find(|link| link.url == url) {
            Some(link) => link.allow_private |= by_user,
            None => wants.links.push(WantedLink { url, allow_private: by_user }),
        }
    }
    wants
}

/// The paths of the image files attached to `post` (L5: previewed through the image scan).
fn image_files(post: &Post) -> impl Iterator<Item = String> + '_ {
    post.extras.files.iter().filter(|file| file_family(&file.mime) == FormatFamily::Image).map(|file| file.path.clone())
}

impl MediaWants {
    /// The URLs, in order.
    pub fn urls(&self) -> impl Iterator<Item = &str> {
        self.links.iter().map(|link| link.url.as_str())
    }

    /// Whether `url` may point at a private address.
    pub fn allows_private(&self, url: &str) -> bool {
        self.links.iter().any(|link| link.url == url && link.allow_private)
    }
}

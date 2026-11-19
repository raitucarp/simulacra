//! Captured posts (pure): a note becomes a thread when it is longer than one post, a link
//! rides on the first post, an image is attached to it; every post is marked as a capture.

use super::capture::Capture;
use crate::domain::post::{PostExtras, PostOrigin};
use crate::domain::{split_into_thread, NetworkId, Post, PostId, ProfileId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

/// What to post: the classified text and an optional image already in the media folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureDraft {
    pub capture: Capture,
    pub image_path: Option<String>,
}

/// The texts of the thread and the link of its first post.
fn parts(capture: &Capture, limit: usize) -> (Vec<String>, Option<String>) {
    match capture {
        Capture::Empty => (Vec::new(), None),
        Capture::Note(text) => (split_into_thread(text, limit), None),
        Capture::Link { url, note } => (split_into_thread(note, limit), Some(url.clone())),
    }
}

/// The posts of one capture, oldest first, each replying to the one before.
pub fn capture_posts(
    draft: &CaptureDraft,
    limit: usize,
    author: &ProfileId,
    network: &NetworkId,
    now: Timestamp,
    mut next_id: impl FnMut() -> PostId,
) -> Result<Vec<Post>> {
    let (mut texts, link) = parts(&draft.capture, limit);
    if texts.is_empty() && (link.is_some() || draft.image_path.is_some()) {
        texts.push(String::new());
    }
    if texts.is_empty() {
        return Err(AppError::EmptyContent);
    }
    let mut posts: Vec<Post> = Vec::with_capacity(texts.len());
    for (index, text) in texts.iter().enumerate() {
        let parent = posts.last().map(|post| post.id.clone());
        let (link, image) = if index == 0 { (link.clone(), draft.image_path.clone()) } else { (None, None) };
        let mut post = Post::create_with_attachments(next_id(), network.clone(), author.clone(), text, link, image, parent, None, None)?;
        post.created_at = now + jiff::SignedDuration::from_millis(index as i64);
        post.extras = PostExtras { origin: Some(PostOrigin::Capture), ..PostExtras::default() };
        if let Some(previous) = posts.last_mut() {
            previous.replies_count = 1;
        }
        posts.push(post);
    }
    Ok(posts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(capture: Capture, image: Option<&str>) -> Result<Vec<Post>> {
        let mut n = 0;
        let draft = CaptureDraft { capture, image_path: image.map(str::to_string) };
        capture_posts(&draft, 280, &ProfileId::new("me"), &NetworkId::new("net"), Timestamp::now(), || {
            n += 1;
            PostId::new(format!("c{n}"))
        })
    }

    #[test]
    fn notes_links_and_images_become_capture_posts() {
        let long = build(Capture::Note("An idea worth keeping. ".repeat(30)), None).expect("thread");
        assert!(long.len() > 1);
        assert_eq!(long[1].reply_to_id.as_ref(), Some(&long[0].id));
        assert!(long.iter().all(|p| p.extras.origin == Some(PostOrigin::Capture)));
        let link = build(Capture::Link { url: "https://example.org".into(), note: String::new() }, None).expect("link");
        assert_eq!((link.len(), link[0].link_url.as_deref(), link[0].content.as_str()), (1, Some("https://example.org"), ""));
        let image = build(Capture::Empty, Some("media/captures/a.png")).expect("image");
        assert_eq!(image[0].image_path.as_deref(), Some("media/captures/a.png"));
        assert_eq!(build(Capture::Empty, None), Err(AppError::EmptyContent));
    }
}

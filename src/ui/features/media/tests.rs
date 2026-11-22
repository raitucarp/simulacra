use super::render_post_media;
use crate::domain::media::ImageAttachment;
use crate::domain::{Post, Profile};
use crate::theme::minimalist_dark;

fn post(content: &str) -> Post {
    Post::create("p", "n", Profile::new_primary("u", "U", "@u"), content).expect("post")
}

#[test]
fn plain_posts_have_no_media_block() {
    let theme = minimalist_dark();
    let plain = post("just words");
    assert!(render_post_media(&plain, &[] as &[Post], &theme, None).is_none());
}

#[test]
fn links_videos_and_galleries_render_without_a_cache() {
    let theme = minimalist_dark();
    for content in ["https://a.org/page", "https://youtu.be/dQw4w9WgXcQ", "https://cdn.org/clip.webm"] {
        assert!(render_post_media(&post(content), &[] as &[Post], &theme, None).is_some(), "{content}");
    }
    let mut gallery = post("a gallery");
    gallery.extras.images = (0..5).map(|i| ImageAttachment::at(format!("missing-{i}.png"))).collect();
    assert!(render_post_media(&gallery, &[] as &[Post], &theme, None).is_some());
    let mut legacy = post("an agent image");
    legacy.image_path = Some("C:/nowhere/agent.png".into());
    assert!(render_post_media(&legacy, &[] as &[Post], &theme, None).is_some());
}

#[test]
fn attached_files_get_a_card_with_their_facts() {
    use crate::domain::media::FileRef;
    let theme = minimalist_dark();
    let mut with_file = post("the report");
    let file = FileRef { size: 1_300_000, modified: Some(crate::time::utc(2026, 11, 2, 12, 0, 0)), mime: "application/pdf".into(), ..FileRef::at("/home/ann/report.pdf") };
    with_file.extras.files.push(file.clone());
    assert!(render_post_media(&with_file, &[] as &[Post], &theme, None).is_some());
    let meta = super::file_card::file_meta(&file, "PDF", None);
    assert!(meta.starts_with("PDF · ") && meta.contains("1.2") && meta.ends_with("2026-11-02"), "{meta}");
    assert_eq!(super::file_card::file_meta(&FileRef::at("/a/b"), "Data", None).matches(" · ").count(), 1, "no date, no separator for it");
}

#[test]
fn image_files_render_their_preview_and_row() {
    use crate::domain::media::FileRef;
    let theme = minimalist_dark();
    let mut with_image = post("a photo");
    with_image.extras.files.push(FileRef { mime: "image/png".into(), ..FileRef::at("/nowhere/photo.png") });
    assert!(render_post_media(&with_image, &[] as &[Post], &theme, None).is_some());
}

#[test]
fn video_files_show_their_poster_or_fall_back_to_the_file_card() {
    use crate::domain::media::files::{FileFacts, FilePreview, FileScan};
    use crate::domain::media::{FileRef, MediaCatalog};
    let theme = minimalist_dark();
    let clip = FileRef { mime: "video/mp4".into(), ..FileRef::at("/home/ann/clip.mp4") };
    let _fallback = super::file_video::video_file(0, &clip, None, &theme);
    let preview = FilePreview::Video { poster: Some("/media/posters/clip.jpg".into()), seconds: Some(95) };
    let catalog = MediaCatalog::default().with_file(clip.path.clone(), FileScan::Present(FileFacts { preview }));
    let _poster = super::file_video::video_file(0, &clip, Some(&catalog), &theme);
    let meta = super::file_card::file_meta(&clip, "Video", Some("1:35".into()));
    assert!(meta.starts_with("Video · ") && meta.ends_with(" · 1:35"), "{meta}");
}

use crate::domain::media::{media_wants, ImageAttachment, WantedLink};
use crate::domain::{Post, Profile};

fn post(id: &str, content: &str) -> Post {
    Post::create(id, "n", Profile::new_primary("u", "U", "@u"), content).expect("post")
}

#[test]
fn collects_first_links_and_images_without_duplicates() {
    let mut a = post("a", "see https://a.org/x and https://b.org/");
    a.extras.images = vec![ImageAttachment::at("one.png"), ImageAttachment::at("two.png")];
    let mut b = post("b", "again https://a.org/x");
    b.image_path = Some("one.png".into());
    let wants = media_wants([(&a, false), (&b, true)], true);
    assert_eq!(wants.links, vec![WantedLink { url: "https://a.org/x".into(), allow_private: true }]);
    assert_eq!(wants.images, vec!["one.png", "two.png"]);
    assert!(wants.allows_private("https://a.org/x") && !wants.allows_private("https://b.org/"));
}

#[test]
fn agent_links_never_reach_private_addresses() {
    let agent_post = post("a", "http://192.168.1.1/admin");
    let wants = media_wants([(&agent_post, false)], true);
    assert_eq!(wants.urls().collect::<Vec<_>>(), ["http://192.168.1.1/admin"]);
    assert!(!wants.allows_private("http://192.168.1.1/admin"));
}

#[test]
fn previews_off_and_video_files_fetch_nothing() {
    let page = post("a", "https://a.org/page");
    assert!(media_wants([(&page, true)], false).links.is_empty());
    let mut file = post("b", "https://cdn.org/clip.mp4");
    file.image_path = Some("still.png".into());
    let wants = media_wants([(&file, true)], true);
    assert!(wants.links.is_empty());
    assert_eq!(wants.images, vec!["still.png"], "images are local and scanned regardless");
}

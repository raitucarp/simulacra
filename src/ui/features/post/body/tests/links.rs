//! L1: web links in the body lay out as link items, show a short form and keep the link card.

use super::super::layout::{layout, Item};
use super::super::link::shown_url;
use crate::domain::media::{media_wants, post_links};
use crate::domain::post::tokenize_post_body;
use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;

#[test]
fn a_written_url_lays_out_as_a_link_item() {
    let tokens = tokenize_post_body("read https://a.org/x, now");
    let items = layout(&tokens);
    assert!(items.contains(&Item::Url { url: "https://a.org/x", trailing: "," }), "{items:?}");
    assert!(items.contains(&Item::Word { text: "now".to_string(), marks: Vec::new() }));
}

#[test]
fn links_show_without_scheme_and_long_ones_are_cut_short() {
    assert_eq!(shown_url("https://a.org/"), "a.org");
    assert_eq!(shown_url("HTTP://a.org/x/"), "a.org/x/");
    let long = format!("https://a.org/{}", "p".repeat(80));
    let shown = shown_url(&long);
    assert!(shown.ends_with('…') && shown.chars().count() == 41, "{shown}");
}

#[test]
fn the_link_colour_is_its_own_readable_token() {
    let theme = ThemeTokens::default();
    assert_ne!(theme.link_text, theme.text_primary);
    assert_ne!(theme.link_text, theme.bg_feed_post);
}

#[test]
fn link_cards_still_render_for_posts_with_url_tokens() {
    let post = Post::create("p", "n", Profile::new_primary("u", "U", "@u"), "see **https://a.org/page**").expect("post");
    assert_eq!(post_links(None, &post.content), vec!["https://a.org/page".to_string()]);
    assert_eq!(media_wants([(&post, true)], true).urls().collect::<Vec<_>>(), vec!["https://a.org/page"]);
    let theme = crate::theme::minimalist_dark();
    assert!(crate::ui::features::media::render_post_media(&post, &[] as &[Post], &theme, None).is_some());
}

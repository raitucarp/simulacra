//! Reactions and claps (R2): adding, removing, clapping any number of times, and totals.

use super::{normalize_emoji, PostReactions, Reaction, ReactionBook, ReactionChip, CLAP};
use crate::domain::{PostId, ProfileId};
use jiff::Timestamp;

const FIRE: &str = "\u{1F525}";
const HEART: &str = "\u{2764}\u{FE0F}";

fn at(minute: i8) -> Timestamp {
    crate::time::utc(2026, 11, 20, 9, minute, 0)
}

fn post() -> PostId {
    PostId::new("p1")
}

fn who(id: &str) -> ProfileId {
    ProfileId::new(id)
}

#[test]
fn an_emoji_counts_once_per_profile_and_can_be_taken_back() {
    let mut set = PostReactions::default();
    let added = set.add(&post(), &who("me"), FIRE, at(1)).expect("added");
    assert_eq!((added.count, added.emoji.as_str()), (1, FIRE));
    assert_eq!(set.add(&post(), &who("me"), FIRE, at(2)), None, "adding it again changes nothing");
    set.add(&post(), &who("agent"), FIRE, at(3));
    assert_eq!(set.chips(), vec![ReactionChip { emoji: FIRE.into(), total: 2, profiles: vec![who("me"), who("agent")] }]);

    let removed = set.remove(&who("me"), FIRE, at(4)).expect("removed");
    assert_eq!((removed.count, removed.updated_at), (0, at(4)), "a removal is saved as a zero count");
    assert_eq!(set.remove(&who("me"), FIRE, at(5)), None);
    assert_eq!(set.chips()[0].profiles, vec![who("agent")]);
}

#[test]
fn toggling_adds_then_removes() {
    let mut set = PostReactions::default();
    assert_eq!(set.toggle(&post(), &who("me"), HEART, at(1)).map(|r| r.count), Some(1));
    assert!(set.has(&who("me"), HEART));
    assert_eq!(set.toggle(&post(), &who("me"), HEART, at(2)).map(|r| r.count), Some(0));
    assert!(set.is_empty());
}

#[test]
fn claps_add_up_without_a_limit_and_come_off_one_at_a_time() {
    let mut set = PostReactions::default();
    for minute in 0..50 {
        set.add(&post(), &who("me"), CLAP, at(minute));
    }
    set.clap(&post(), &who("me"), 950, at(51));
    assert_eq!(set.claps_by(&who("me")), 1000, "clapping is not capped");
    set.clap(&post(), &who("agent"), u32::MAX, at(52));
    set.clap(&post(), &who("agent"), 5, at(53));
    assert_eq!(set.claps_by(&who("agent")), u32::MAX, "counts saturate instead of wrapping");
    assert_eq!(set.claps(), u32::MAX);
    assert_eq!(set.clap(&post(), &who("me"), 0, at(54)), None);

    let left = set.unclap(&who("me"), 1, at(55)).expect("one fewer");
    assert_eq!(left.count, 999);
    let gone = set.unclap(&who("me"), 5000, at(56)).expect("all gone");
    assert_eq!(gone.count, 0);
    assert_eq!(set.claps_by(&who("me")), 0);
    assert_eq!(set.unclap(&who("me"), 1, at(57)), None, "nothing left to take back");
    assert!(set.chips().is_empty(), "claps are not an emoji chip");
}

#[test]
fn per_profile_totals_count_emoji_once_and_every_clap() {
    let mut set = PostReactions::default();
    set.add(&post(), &who("me"), FIRE, at(1));
    set.clap(&post(), &who("agent"), 3, at(2));
    set.add(&post(), &who("me"), HEART, at(3));
    set.clap(&post(), &who("me"), 2, at(4));
    assert_eq!(set.profile_totals(), vec![(who("me"), 4), (who("agent"), 3)]);
    assert_eq!(set.clappers(), vec![(who("agent"), 3), (who("me"), 2)]);
    assert_eq!(set.emoji_of(&who("me")), vec![FIRE, HEART]);
    let order: Vec<String> = set.chips().into_iter().map(|chip| chip.emoji).collect();
    assert_eq!(order, vec![FIRE.to_string(), HEART.to_string()], "chips keep the order emoji were first given");
}

#[test]
fn the_book_groups_rows_by_post_and_drops_empty_posts() {
    let row = |post: &str, profile: &str, emoji: &str, count: u32| Reaction {
        post_id: PostId::new(post), profile_id: who(profile), emoji: emoji.into(), count, updated_at: at(0),
    };
    let mut book = ReactionBook::from_rows([row("p1", "me", FIRE, 1), row("p2", "me", CLAP, 4), row("p1", "agent", CLAP, 2), row("p3", "me", FIRE, 0)]);
    assert_eq!(book.of(&PostId::new("p1")).map(|set| set.rows().len()), Some(2));
    assert!(book.of(&PostId::new("p3")).is_none(), "zero counts are not kept");
    assert_eq!(book.given_by(&who("me")).count(), 2);

    book.unclap(&PostId::new("p2"), &who("me"), 4, at(1));
    assert!(book.of(&PostId::new("p2")).is_none(), "a post with nothing left has no entry");
    book.put(row("p2", "agent", HEART, 1));
    book.put(row("p1", "me", FIRE, 0));
    assert_eq!(book.rows().count(), 2);
    assert!(book.forget(&PostId::new("p1")) && book.rows().count() == 1);
}

#[test]
fn only_short_pictographic_text_is_an_emoji() {
    assert_eq!(normalize_emoji(" \u{1F44D} ").as_deref(), Some("\u{1F44D}"));
    assert_eq!(normalize_emoji("1\u{FE0F}\u{20E3}").as_deref(), Some("1\u{FE0F}\u{20E3}"), "keycaps are emoji");
    let family = "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}\u{1F3FC}";
    assert!(normalize_emoji(family).is_some());
    for bad in ["", "   ", "lol", ":)", "\u{1F525} \u{1F525}", "1", "\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}\u{1F525}"] {
        assert_eq!(normalize_emoji(bad), None, "{bad:?}");
    }
}

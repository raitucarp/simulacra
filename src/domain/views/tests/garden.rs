//! N3: the garden view's beds.

use crate::domain::crypto::KdfParams;
use crate::domain::post::garden::{GardenNote, Maturity};
use crate::domain::post::lock::LockPassword;
use crate::domain::views::garden::{garden, note_count};
use crate::domain::Post;

fn note(id: &str, text: &str, maturity: Maturity, day: i8) -> Post {
    let mut post = Post::create(id, "n1", "u1", text).expect("valid");
    let planted = crate::time::utc(2026, 11, 1, 9, 0, 0);
    post.extras.garden = Some(GardenNote::planted(planted).tended(maturity, crate::time::utc(2026, 11, day, 9, 0, 0)));
    post
}

#[test]
fn notes_sit_in_beds_by_maturity_evergreen_first_most_recently_tended_first() {
    let posts = vec![note("a", "seed one", Maturity::Seed, 1), note("b", "old evergreen", Maturity::Evergreen, 2), note("c", "new evergreen", Maturity::Evergreen, 9), Post::create("d", "n1", "u1", "stream only").expect("valid")];
    let beds = garden(&posts);
    let shape: Vec<(Maturity, Vec<&str>)> = beds.iter().map(|bed| (bed.maturity, bed.notes.iter().map(|n| n.post_id.as_str()).collect())).collect();
    assert_eq!(shape, vec![(Maturity::Evergreen, vec!["c", "b"]), (Maturity::Seed, vec!["a"])]);
    assert_eq!(note_count(&beds), 3);
}

#[test]
fn a_locked_note_shows_no_text() {
    let mut locked = note("l", "Quetzal orchard secret", Maturity::Budding, 3);
    locked.lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    let beds = garden([&locked]);
    let entry = &beds[0].notes[0];
    assert!(entry.locked && entry.text.is_empty());
}

#[test]
fn every_stage_has_its_art_and_name() {
    let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    for maturity in Maturity::ALL {
        assert!(assets.join(maturity.art_path()).is_file(), "{maturity:?}");
        let key = format!("posts.garden.stage.{}", maturity.key());
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
    for key in ["views.garden.show", "views.garden.empty", "views.garden.planted", "views.garden.replies_one", "views.garden.replies_other"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}

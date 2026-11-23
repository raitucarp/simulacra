//! N16–N20 on the post card: what each line shows, and never past a lock.

use crate::domain::crypto::KdfParams;
use crate::domain::post::lens::{Lens, MAX_LENSES};
use crate::domain::post::lock::LockPassword;
use crate::domain::Post;
use crate::ui::features::data::{lens_name, LensDialog};

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

/// A locked post that carries `finish`'s marks anyway (as a damaged or imported one might).
pub(super) fn forged(finish: impl FnOnce(&mut Post)) -> Post {
    let mut post = Post::create("p", "n1", "u1", "sealed words").expect("post");
    post.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    finish(&mut post);
    post
}

#[test]
fn a_lens_reply_names_its_lens_but_never_past_a_lock() {
    let mut post = Post::create("p", "n1", "u1", "Whose streets?").expect("post");
    post.extras.lens = Some(Lens::Ethics);
    assert!(super::lens_chip::lens_chip_text(&post).is_some_and(|text| text.contains(&lens_name(Lens::Ethics))));
    assert!(super::lens_chip::lens_chip_text(&forged(|post| post.extras.lens = Some(Lens::Ethics))).is_none());
    for lens in Lens::ALL {
        let key = format!("posts.lens.name_{}", lens.key());
        assert_ne!(lens_name(*lens), key, "{key} is translated");
    }
}

#[test]
fn the_lens_sheet_starts_with_the_defaults_and_never_picks_past_the_limit() {
    let mut dialog = LensDialog::new(crate::domain::PostId::new("p"), 3);
    assert_eq!(dialog.chosen, Lens::DEFAULTS.to_vec());
    dialog.toggle(Lens::Ethics);
    assert!(!dialog.chosen.contains(&Lens::Ethics));
    for lens in Lens::ALL {
        dialog.toggle(*lens);
    }
    assert!(dialog.chosen.len() <= MAX_LENSES && dialog.problem.is_some(), "a fifth lens is refused with a reason");
}

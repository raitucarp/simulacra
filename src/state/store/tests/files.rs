//! L11: relinking a moved file points the post at it, saved with the post and never a new
//! version; only files the user's own posts have.

use crate::domain::media::FileRef;
use crate::state::store::AppState;
use crate::storage::{Database, Vault};

fn slides(path: &str) -> FileRef {
    FileRef { size: 10, mime: "application/pdf".into(), sha256: "ab".repeat(32), ..FileRef::at(path) }
}

#[test]
fn a_relinked_file_is_saved_with_the_post_without_a_version() {
    let vault = Vault::spawn(Database::in_memory().expect("in-memory db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let id = state.submit_post("the slides").unwrap();
    let position = state.post_position(&id).unwrap();
    state.posts[position].extras.files.push(slides("/old/slides.pdf"));
    state.relink_file(&id, 0, slides("/new/slides.pdf")).expect("relinked");
    assert_eq!(state.get_post(&id).unwrap().extras.files, vec![slides("/new/slides.pdf")]);
    assert!(state.post_revisions(&id).is_empty(), "a relink is no new version");
    let reloaded = AppState::load_from_db(&vault).expect("reload");
    assert_eq!(reloaded.get_post(&id).unwrap().extras.files[0].path, "/new/slides.pdf");
}

#[test]
fn only_files_a_post_has_can_be_relinked() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("no files").unwrap();
    assert!(state.relink_file(&id, 0, slides("/a.pdf")).is_err());
    assert!(state.get_post(&id).unwrap().extras.files.is_empty());
}

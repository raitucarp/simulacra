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

#[test]
fn files_from_the_composer_go_on_the_first_post_even_without_text() {
    use crate::state::store::{ThreadAnchor, ThreadDraft};
    let mut state = AppState::new_in_memory();
    let drafts = [ThreadDraft { files: vec![slides("/talks/slides.pdf")], ..ThreadDraft::text("") }, ThreadDraft::text("and a reply")];
    let ids = state.submit_thread(&drafts, &ThreadAnchor::default()).expect("a file alone makes a post");
    assert_eq!(state.get_post(&ids[0]).unwrap().extras.files, vec![slides("/talks/slides.pdf")]);
    assert!(state.get_post(&ids[1]).unwrap().extras.files.is_empty());
    assert!(state.submit_thread(&[ThreadDraft::text("  ")], &ThreadAnchor::default()).is_err(), "nothing at all is still refused");
}

#[test]
fn an_edit_changes_the_files_and_each_version_keeps_its_own() {
    use crate::domain::data::history::{Attachments, PostRecord};
    use crate::state::store::PostEdit;
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("the slides").unwrap();
    let look = |path: &str| Some(PostRecord { attachments: Some(Attachments::new(None, Vec::new()).with_files(vec![slides(path)])), ..PostRecord::default() });
    let at = crate::time::utc(2026, 11, 21, 9, 0, 0);
    state.edit_post_version(&id, &PostEdit { look: look("/v2/slides.pdf"), ..PostEdit::text("the slides") }, at).unwrap().expect("a version");
    assert_eq!(state.get_post(&id).unwrap().extras.files, vec![slides("/v2/slides.pdf")]);
    let record = PostRecord::of(state.get_post(&id).unwrap());
    let json = record.to_json();
    assert!(json.contains("/v2/slides.pdf"), "{json}");
    assert_eq!(PostRecord::parse(&json), Some(record));
    let older: PostRecord = serde_json::from_str(r#"{"attachments":{"link_url":"https://a.org"}}"#).unwrap();
    assert!(older.attachments.is_some_and(|kept| kept.files.is_empty()), "records kept before files read as none");
}

//! Local file attachments: the persisted reference and its defaults.

use super::*;
use crate::domain::post::PostExtras;

fn report() -> FileRef {
    FileRef {
        path: r"C:\Users\ann\Reports\Q3 summary.PDF".to_string(),
        size: 1_234_567,
        modified: Some(crate::time::utc(2026, 11, 2, 9, 30, 0)),
        mime: "application/pdf".to_string(),
        sha256: "ab".repeat(32),
    }
}

#[test]
fn a_file_ref_round_trips_through_the_extras_column() {
    let mut extras = PostExtras::default();
    extras.files.push(report());
    let json = serde_json::to_string(&extras).unwrap();
    assert!(json.contains("\"files\"") && json.contains("Q3 summary.PDF"), "{json}");
    assert_eq!(serde_json::from_str::<PostExtras>(&json).unwrap(), extras);
}

#[test]
fn missing_fields_read_as_defaults() {
    let extras: PostExtras = serde_json::from_str(r#"{"mood":"calm"}"#).unwrap();
    assert!(extras.files.is_empty());
    let file: FileRef = serde_json::from_str(r#"{"path":"/home/ann/notes.txt"}"#).unwrap();
    assert_eq!(file, FileRef::at("/home/ann/notes.txt"));
    assert_eq!(serde_json::to_string(&file).unwrap(), r#"{"path":"/home/ann/notes.txt","size":0}"#);
}

#[test]
fn files_alone_are_not_details_and_empty_extras_stay_empty() {
    let mut extras = PostExtras::default();
    assert!(extras.is_empty());
    extras.files.push(FileRef::at("/a/b.txt"));
    assert!(!extras.is_empty() && !extras.has_details());
}

#[test]
fn names_and_extensions_come_from_either_separator() {
    assert_eq!(report().name(), "Q3 summary.PDF");
    assert_eq!(report().extension().as_deref(), Some("pdf"));
    assert_eq!(FileRef::at("/home/ann/.bashrc").extension(), None);
    assert_eq!(FileRef::at("/home/ann/Makefile").name(), "Makefile");
    assert_eq!(FileRef::at("/home/ann/dir/").name(), "dir");
}

#[test]
fn a_post_with_a_file_has_an_attachment() {
    let mut post = crate::domain::Post::create("p", "n", crate::domain::Profile::new_primary("u", "U", "@u"), "see the file").unwrap();
    assert!(!post.has_attachments());
    post.extras.files.push(report());
    assert!(post.has_attachments());
}

use super::labels::{family_art, position_label};
use super::session::{ImportSession, ImportStage};
use crate::domain::preferences::PostIdentity;
use crate::import::{load_bytes, FormatFamily, Granularity, UnitPosition};

const BOOK: &str = "<html><body><h1>One</h1><p>First. Second.</p><p>Third.</p><h1>Two</h1><p>Fourth.</p></body></html>";

fn ready(granularity: Granularity) -> ImportSession {
    let mut session = ImportSession::reading("book.html", granularity, PostIdentity::Yourself, 280);
    assert_eq!(session.stage, ImportStage::Reading);
    session.load(load_bytes("book.html", None, BOOK.as_bytes().to_vec()));
    session
}

#[test]
fn loading_segments_and_selects_everything() {
    let session = ready(Granularity::Paragraph);
    assert_eq!(session.stage, ImportStage::Ready);
    assert_eq!(session.units.len(), 3);
    assert_eq!(session.selected_count(), 3);
    assert_eq!(session.counts.posts, 3);
    assert_eq!(session.chapters_with_units(), [0, 1]);
    assert_eq!(session.chapter_titles(), [Some("One".to_string()), Some("Two".to_string())]);
}

#[test]
fn granularity_selection_and_chapters_update_the_counts() {
    let mut session = ready(Granularity::Paragraph);
    session.set_granularity(Granularity::Sentence);
    assert_eq!(session.units.len(), 4);
    session.toggle(0);
    assert_eq!((session.selected_count(), session.counts.units), (3, 3));
    session.select_chapter(1);
    assert_eq!(session.selected_units().iter().map(|u| u.text.as_str()).collect::<Vec<_>>(), ["Fourth."]);
    session.select_all(false);
    assert_eq!(session.counts.posts, 0);
    session.toggle(99);
    session.set_granularity(Granularity::Chapter);
    assert_eq!(session.units.len(), 2);
    assert_eq!(session.selected_count(), 2);
}

#[test]
fn failures_are_kept_and_labels_read_well() {
    let mut session = ImportSession::reading("x.bin", Granularity::Paragraph, PostIdentity::Anonymous, 280);
    session.fail("too large".into());
    assert_eq!(session.stage, ImportStage::Failed("too large".into()));
    assert!(family_art(FormatFamily::Book).ends_with("import/book.svg"));
    let position = UnitPosition { chapter: 0, paragraph: Some(1), sentence: Some(2) };
    assert!(position_label(position, Some("Loomings")).contains("Loomings"));
}

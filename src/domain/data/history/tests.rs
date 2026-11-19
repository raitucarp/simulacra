use super::*;
use jiff::Timestamp;

fn at(hour: i8) -> Timestamp {
    crate::time::utc(2026, 11, 18, hour, 0, 0)
}

fn rendered(ops: &[DiffOp]) -> String {
    ops.iter()
        .map(|op| match op {
            DiffOp::Same(t) => t.clone(),
            DiffOp::Added(t) => format!("[+{t}]"),
            DiffOp::Removed(t) => format!("[-{t}]"),
        })
        .collect()
}

#[test]
fn tokens_split_words_spaces_and_marks() {
    assert_eq!(tokens("Hello,  world! it's"), vec!["Hello", ",", "  ", "world", "!", " ", "it's"]);
    assert_eq!(tokens("[[Spinoza]]"), vec!["[", "[", "Spinoza", "]", "]"]);
    assert!(tokens("").is_empty());
    assert_eq!(tokens("日本語 テキスト"), vec!["日本語", " ", "テキスト"]);
}

#[test]
fn identical_texts_diff_to_one_same_run() {
    assert_eq!(word_diff("one two", "one two"), vec![DiffOp::Same("one two".into())]);
    assert!(word_diff("", "").is_empty());
}

#[test]
fn diff_marks_insertions_and_deletions_by_word() {
    let ops = word_diff("The mind is a mirror.", "The mind is a lamp, not a mirror.");
    assert_eq!(rendered(&ops), "The mind is a [+lamp, not a ]mirror.");
    let ops = word_diff("Reason is slow and sure", "Reason is sure");
    assert_eq!(rendered(&ops), "Reason is [-slow and ]sure");
}

#[test]
fn replaced_words_show_removal_before_addition() {
    let ops = word_diff("a quick fox", "a slow fox");
    assert_eq!(rendered(&ops), "a [-quick][+slow] fox");
    assert_eq!(diff_stats(&ops), DiffStats { added: 1, removed: 1 });
}

#[test]
fn diff_reassembles_both_texts() {
    let (old, new) = ("Socrates asked; Spinoza answered with a proof.", "Spinoza asked, and Socrates answered without proof!");
    let ops = word_diff(old, new);
    let before: String = ops.iter().filter_map(|op| match op { DiffOp::Added(_) => None, DiffOp::Same(t) | DiffOp::Removed(t) => Some(t.as_str()) }).collect();
    let after: String = ops.iter().filter_map(|op| match op { DiffOp::Removed(_) => None, DiffOp::Same(t) | DiffOp::Added(t) => Some(t.as_str()) }).collect();
    assert_eq!((before.as_str(), after.as_str()), (old, new));
}

#[test]
fn huge_rewrites_fall_back_to_delete_and_insert() {
    let old = "alpha ".repeat(1200).trim_end().to_string();
    let new = "beta ".repeat(1200).trim_end().to_string();
    let ops = word_diff(&old, &new);
    assert_eq!(ops.len(), 2);
    assert!(matches!(ops[0], DiffOp::Removed(_)) && matches!(ops[1], DiffOp::Added(_)));
}

#[test]
fn stats_count_words_only() {
    let ops = word_diff("x", "x, y and z!");
    assert_eq!(diff_stats(&ops), DiffStats { added: 3, removed: 0 });
}

#[test]
fn subjects_round_trip_their_stored_parts() {
    for subject in [RevisionSubject::Post(PostId::new("post_1")), RevisionSubject::Document("doc_1".into())] {
        assert_eq!(RevisionSubject::from_parts(subject.kind(), subject.id()), Some(subject.clone()));
    }
    assert_eq!(RevisionSubject::from_parts("unknown", "x"), None);
}

#[test]
fn version_list_puts_current_first_and_marks_the_original() {
    let subject = RevisionSubject::Post(PostId::new("p"));
    let first = Revision { saved_at: at(9), ..Revision::of(subject.clone(), "draft one", at(9)) };
    let second = Revision::of(subject, "draft two", at(11));
    let list = version_list("", "final", &[first, second]);
    let kinds: Vec<VersionKind> = list.iter().map(|v| v.kind).collect();
    assert_eq!(kinds, vec![VersionKind::Current, VersionKind::Earlier, VersionKind::Original]);
    let texts: Vec<&str> = list.iter().map(|v| v.content.as_str()).collect();
    assert_eq!(texts, vec!["final", "draft two", "draft one"]);
    assert_eq!(list[0].replaced_at, None);
}

#[test]
fn whitespace_only_changes_are_not_edits() {
    assert!(!is_real_edit("same text ", "  same text"));
    assert!(is_real_edit("same text", "same text."));
}

#[test]
fn revisions_carry_document_records() {
    let revision = Revision::of(RevisionSubject::Document("doc".into()), "body", at(1)).with_record("Title", "{}".into());
    assert_eq!((revision.title.as_str(), revision.record_json.as_str()), ("Title", "{}"));
    assert!(revision.id.as_str().starts_with(REVISION_ID_PREFIX));
}

#[test]
fn editing_a_post_rederives_links_and_mentions() {
    let mut post = crate::domain::Post::create("p", "net", "user_main", "Hello @ada about [[q1]]").unwrap();
    post.reply_to_id = Some(PostId::new("parent"));
    let edited = edited_post(&post, "  Now about [[q2]] and @bo ");
    assert_eq!(edited.content, "Now about [[q2]] and @bo");
    assert!(edited.linked_posts.contains(&PostId::new("q2")) && !edited.linked_posts.contains(&PostId::new("q1")));
    assert!(edited.linked_posts.contains(&PostId::new("parent")));
    assert_eq!((edited.id.clone(), edited.created_at), (post.id.clone(), post.created_at));
    assert_ne!(edited.mentions, post.mentions);
}

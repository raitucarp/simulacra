use super::index::index_letter;
use super::{subject_index, Subject, SubjectTree};
use crate::domain::document::fixtures::{library, LANGUAGE, METAPHYSICS, PHILOSOPHY};
use crate::domain::document::SubjectId;
use std::collections::{HashMap, HashSet};

fn ids(list: &[SubjectId]) -> Vec<&str> {
    list.iter().map(SubjectId::as_str).collect()
}

#[test]
fn the_tree_nests_narrower_terms_under_broader_ones_sorted_by_label() {
    let tree = library().tree();
    assert_eq!(tree.roots.len(), 1);
    let labels: Vec<&str> = tree.roots[0].children.iter().map(|n| n.subject.label.as_str()).collect();
    assert_eq!(labels, ["Ethics", "Metaphysics", "Philosophy of language"]);
    let mut below = tree.descendants(&SubjectId::new(PHILOSOPHY));
    below.sort();
    assert_eq!(ids(&below), ["subj_ethics", "subj_language", "subj_metaphysics", "subj_philosophy"]);
    let path: Vec<&str> = tree.path(&SubjectId::new(METAPHYSICS)).iter().map(|s| s.label.as_str()).collect();
    assert_eq!(path, ["Philosophy", "Metaphysics"]);
}

#[test]
fn unknown_broader_terms_and_loops_become_top_terms() {
    let subjects = vec![
        Subject::new("a", "Alpha").under("b"),
        Subject::new("b", "Beta").under("a"),
        Subject::new("c", "Gamma").under("a"),
        Subject::new("d", "Delta").under("missing"),
        Subject::new("e", "Self").under("e"),
    ];
    let tree = SubjectTree::build(&subjects);
    let roots: Vec<&str> = tree.roots.iter().map(|n| n.subject.id.as_str()).collect();
    assert_eq!(roots, ["a", "b", "d", "e"]);
    assert_eq!(ids(&tree.descendants(&SubjectId::new("a"))), ["a", "c"]);
}

#[test]
fn only_expanded_terms_show_their_children() {
    let tree = library().tree();
    assert_eq!(tree.visible_rows(&HashSet::new()).len(), 1);
    let open: HashSet<SubjectId> = [SubjectId::new(PHILOSOPHY)].into();
    let rows = tree.visible_rows(&open);
    assert_eq!(rows.len(), 4);
    assert!(rows[0].has_children && rows[0].depth == 0);
    assert!(rows[1..].iter().all(|row| row.depth == 1 && !row.has_children));
}

#[test]
fn the_index_groups_terms_by_initial_with_counts_and_broader_context() {
    let mut subjects = library().subjects;
    subjects.push(Subject::new("subj_20c", "20th-century thought"));
    let counts: HashMap<SubjectId, usize> = [(SubjectId::new(LANGUAGE), 2)].into();
    let index = subject_index(&subjects, &counts);
    let letters: Vec<char> = index.iter().map(|group| group.letter).collect();
    assert_eq!(letters, ['E', 'M', 'P', '#']);
    let p = &index[2];
    assert_eq!(p.entries.iter().map(|e| e.label.as_str()).collect::<Vec<_>>(), ["Philosophy", "Philosophy of language"]);
    assert_eq!((p.entries[1].count, p.entries[1].broader.as_deref()), (2, Some("Philosophy")));
    assert_eq!(index_letter("épistémè"), 'É');
}

#[test]
fn labels_are_looked_up_case_insensitively() {
    let library = library();
    assert_eq!(library.subject_by_label("  metaphysics ").map(|s| s.id.as_str()), Some(METAPHYSICS));
    assert!(library.subject_by_label("Logic").is_none());
}

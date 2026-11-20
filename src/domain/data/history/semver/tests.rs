use super::*;
use crate::domain::data::history::DiffStats;

/// `(why, old, new, expected)` edits of real posts and notes.
const EDITS: &[(&str, &str, &str, Bump)] = &[
    ("identical", "Habits compound.", "Habits compound.", Bump::Patch),
    ("typo", "Teh mind is a mirror, not a lamp.", "The mind is a mirror, not a lamp.", Bump::Patch),
    ("swapped word", "Reason is slow and sure.", "Reason is slow but sure.", Bump::Patch),
    ("missing not", "Memory is a record.", "Memory is not a record.", Bump::Patch),
    ("capitalisation", "spinoza thought god was nature.", "Spinoza thought God was nature.", Bump::Patch),
    ("punctuation", "Habits compound", "Habits compound.", Bump::Patch),
    ("whitespace", "Habits  compound.\n", "Habits compound.", Bump::Patch),
    ("typo in a two-word post", "Hello wrold", "Hello world", Bump::Patch),
    ("both texts empty", "", "", Bump::Patch),
    (
        "a sentence appended",
        "The mind is a mirror.",
        "The mind is a mirror. It shows what we bring to it, not what is there.",
        Bump::Minor,
    ),
    (
        "clauses inserted",
        "Reading slowly is a form of respect.",
        "Reading slowly, with a pencil in hand, is a form of respect for the author.",
        Bump::Minor,
    ),
    (
        "partly reworded",
        "Notes are not storage. They are a conversation with a future self who has forgotten the context.",
        "Notes are not storage. They are letters to a future self who has lost the thread and needs the context.",
        Bump::Minor,
    ),
    (
        "a clause cut",
        "Reading slowly, with a pencil in hand, is a form of respect for the author.",
        "Reading slowly is a form of respect for the author.",
        Bump::Minor,
    ),
    ("first words of an empty draft", "", "First thought.", Bump::Minor),
    ("rewritten", "The mind is a mirror.", "Attention is the rarest form of generosity.", Bump::Major),
    ("the only word replaced", "Yes.", "No.", Bump::Major),
    (
        "cut to the first sentence",
        "Habits compound. Small daily practice beats rare heroic effort, and the gap widens every year.",
        "Habits compound.",
        Bump::Major,
    ),
    ("emptied", "Habits compound.", "", Bump::Major),
];

#[test]
fn edits_bump_by_how_much_of_the_text_they_change() {
    for (why, old, new, expected) in EDITS {
        assert_eq!(Bump::between(old, new), *expected, "{why}: {old:?} -> {new:?}");
    }
}

#[test]
fn thresholds_sit_on_their_boundaries() {
    let stats = |added, removed| DiffStats { added, removed };
    let cases = [
        (stats(PATCH_MAX_WORDS, PATCH_MAX_WORDS), 10, Bump::Patch),
        (stats(PATCH_MAX_WORDS + 1, 0), 10, Bump::Minor),
        (stats(0, PATCH_MAX_WORDS + 1), 10, Bump::Minor),
        (stats(PATCH_MAX_WORDS, PATCH_MAX_WORDS), PATCH_MAX_WORDS + 1, Bump::Patch),
        (stats(0, 4), 8, Bump::Major),
        (stats(0, 4), 9, Bump::Minor),
        (stats(20, 10), 20, Bump::Major),
        (stats(1, 1), 1, Bump::Major),
        (stats(1, 0), 0, Bump::Minor),
        (stats(0, 0), 0, Bump::Patch),
    ];
    for (stats, old_words, expected) in cases {
        assert_eq!(Bump::from_stats(stats, old_words), expected, "{stats:?} of {old_words} words");
    }
    assert!(MAJOR_REMOVED_PERCENT <= 100);
}

#[test]
fn versions_parse_and_print_as_major_minor_patch() {
    for (text, version) in [("1.2.3", SemVer::new(1, 2, 3)), (" 10.0.7 ", SemVer::new(10, 0, 7)), ("0.0.0", SemVer::new(0, 0, 0))] {
        assert_eq!(text.parse::<SemVer>(), Ok(version), "{text:?}");
        assert_eq!(version.to_string().parse::<SemVer>(), Ok(version));
    }
    assert_eq!(SemVer::new(4, 20, 1).to_string(), "4.20.1");
    assert_eq!(SemVer::default(), SemVer::ORIGINAL);
    assert_eq!(SemVer::ORIGINAL.to_string(), "1.0.0");
}

#[test]
fn malformed_versions_are_rejected() {
    for text in ["", "1", "1.2", "1.2.3.4", "v1.2.3", "01.2.3", "1.02.3", "1.-2.3", "+1.2.3", "1.2.x", "1..3", "1.2.3-beta", "4294967296.0.0"] {
        assert_eq!(text.parse::<SemVer>(), Err(ParseSemVerError), "{text:?}");
    }
}

#[test]
fn bumping_resets_the_lower_parts() {
    let version = SemVer::new(1, 2, 3);
    assert_eq!(version.bumped(Bump::Patch), SemVer::new(1, 2, 4));
    assert_eq!(version.bumped(Bump::Minor), SemVer::new(1, 3, 0));
    assert_eq!(version.bumped(Bump::Major), SemVer::new(2, 0, 0));
    assert_eq!(SemVer::new(1, 1, u32::MAX).bumped(Bump::Patch), SemVer::new(1, 1, u32::MAX));
}

#[test]
fn versions_order_numerically() {
    assert!(SemVer::new(1, 10, 0) > SemVer::new(1, 9, 9));
    assert!(SemVer::new(2, 0, 0) > SemVer::new(1, 99, 99));
    assert!(Bump::Patch < Bump::Minor && Bump::Minor < Bump::Major);
}

#[test]
fn a_post_history_numbers_its_versions() {
    let drafts = ["Teh mind is a mirror.", "The mind is a mirror.", "The mind is a mirror. It shows what we bring to it.", "Attention is the rarest form of generosity."];
    let version = drafts.windows(2).fold(SemVer::ORIGINAL, |version, pair| version.bumped(Bump::between(pair[0], pair[1])));
    assert_eq!(version, SemVer::new(2, 0, 0));
    let before_rewrite = drafts[..3].windows(2).fold(SemVer::ORIGINAL, |version, pair| version.bumped(Bump::between(pair[0], pair[1])));
    assert_eq!(before_rewrite, SemVer::new(1, 1, 0));
}

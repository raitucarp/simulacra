use super::*;
use crate::domain::post::correction::{Correction, CorrectionKind};

fn words() -> DistillWords {
    DistillWords { origin: "Distilled from a thread of 2 posts.".into(), summary_heading: "Summary".into(), open_post: "Open the post".into(), retracted: "Retracted".into(), corrected: "Corrected".into(), untitled: "A thread".into(), locked: "(locked)".into(), restricted: "(not shared with everyone)".into() }
}

fn part(id: &str, text: &str) -> DistillPost {
    DistillPost { link: post_link("net", id), author: "Ada".into(), date: "2 Nov 2026".into(), text: text.into(), correction: None }
}

#[test]
fn a_thread_becomes_markdown_with_a_link_back_to_each_post() {
    let mut second = part("p2", "Then measure it twice.");
    second.correction = Some(Correction { kind: CorrectionKind::Corrected, reason: "Three times.".into(), at: jiff::Timestamp::UNIX_EPOCH, told: Vec::new() });
    let doc = thread_markdown("Measuring", &[part("p1", "Measure once."), second], Some("Measure more than you think."), &words());
    assert!(doc.starts_with("# Measuring\n\n*Distilled from a thread of 2 posts.*\n\n## Summary\n\nMeasure more than you think.\n"));
    assert!(doc.contains("## Ada · 2 Nov 2026\n\nMeasure once.\n\n[Open the post](simulacra://net/post/p1)\n"));
    assert!(doc.contains("> **Corrected:** Three times.\n\nThen measure it twice."));
    assert!(doc.ends_with("(simulacra://net/post/p2)\n"));
    assert!(!thread_markdown("T", &[part("p1", "x")], Some("  "), &words()).contains("## Summary"), "no summary, no heading");
}

#[test]
fn titles_come_from_the_first_line_cut_at_a_word() {
    assert_eq!(title_from("\n  Why measure twice\nbecause"), "Why measure twice");
    let long = "word ".repeat(30);
    let title = title_from(&long);
    assert!(title.ends_with('…') && title.chars().count() <= MAX_TITLE_CHARS + 1);
    assert_eq!(title_from("   "), "");
}

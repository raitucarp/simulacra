use super::*;
use crate::domain::post::PostOrigin;
use crate::domain::preferences::PostIdentity;
use crate::domain::{NetworkId, PostId, ProfileId};
use crate::import::segment::{Unit, UnitPosition};

fn unit(chapter: usize, paragraph: usize, sentence: Option<usize>, text: &str) -> Unit {
    Unit { position: UnitPosition { chapter, paragraph: Some(paragraph), sentence }, text: text.to_string() }
}

fn source() -> SourceInfo {
    SourceInfo { document_id: "doc_1".into(), title: "Moby Dick".into(), chapter_titles: vec![Some("Loomings".into()), None] }
}

#[test]
fn citations_name_the_document_chapter_and_paragraph() {
    let cited = cite(&unit(0, 4, None, "x"), &source());
    assert_eq!(cited.id, "doc_1");
    assert_eq!(cited.title, "Moby Dick · Loomings · ¶5");
    let locator = cited.locator.expect("locator");
    assert_eq!((locator.chapter, locator.paragraph, locator.sentence), (1, Some(5), None));
    assert_eq!(cite(&unit(1, 0, Some(2), "x"), &source()).title, "Moby Dick · §2 · ¶1.3");
    let chapter = Unit { position: UnitPosition { chapter: 0, ..UnitPosition::default() }, text: "x".into() };
    assert_eq!(cite(&chapter, &source()).title, "Moby Dick · Loomings");
}

#[test]
fn long_units_become_threads_and_order_is_kept() {
    let long = "word ".repeat(120);
    let units = [unit(0, 0, None, "Short first."), unit(0, 1, None, &long), unit(1, 0, None, "   ")];
    let plan = plan_posts(&units, &source(), 280);
    assert_eq!(plan.drafts.len(), 2, "blank units are dropped");
    assert_eq!(plan.drafts[0].thread, ["Short first."]);
    assert!(plan.drafts[1].thread.len() >= 2);
    assert!(plan.drafts[1].thread.iter().all(|post| post.chars().count() <= 280));
    let run = dry_run(&plan);
    assert_eq!(run.units, 2);
    assert_eq!(run.threads, 1);
    assert_eq!(run.posts, 1 + plan.drafts[1].thread.len());
    assert_eq!(effective_post_limit(5000), crate::domain::MAX_POST_CHARS);
    assert_eq!(effective_post_limit(140), 140);
}

#[test]
fn posts_chain_threads_rise_in_time_and_cite_their_source() {
    let long = "Sentence number one is here. ".repeat(20);
    let plan = plan_posts(&[unit(0, 0, None, &long), unit(0, 1, None, "Next.")], &source(), 280);
    let mut counter = 0;
    let start = crate::time::utc(2026, 1, 1, 0, 0, 0);
    let posts = build_posts(&plan, &ProfileId::new("p"), &NetworkId::new("n"), start, PostOrigin::Import, || {
        counter += 1;
        PostId::new(format!("post_{counter}"))
    })
    .expect("valid posts");
    let thread_len = plan.drafts[0].thread.len();
    assert_eq!(posts.len(), thread_len + 1);
    assert_eq!(posts[0].reply_to_id, None);
    assert_eq!(posts[1].reply_to_id.as_ref(), Some(&posts[0].id));
    assert_eq!(posts[0].replies_count, 1);
    assert_eq!(posts[thread_len - 1].replies_count, 0);
    assert_eq!(posts[thread_len].reply_to_id, None, "the next unit starts a new root");
    assert!(posts.windows(2).all(|pair| pair[0].created_at < pair[1].created_at));
    assert!(posts.iter().all(|post| post.extras.origin == Some(PostOrigin::Import) && post.extras.sources.len() == 1));
}

#[test]
fn author_choices_follow_the_identity_preference() {
    assert_eq!(AuthorChoice::from_identity(PostIdentity::Pseudonym, "  Publius "), AuthorChoice::Pseudonym("Publius".into()));
    assert_eq!(AuthorChoice::from_identity(PostIdentity::Anonymous, "x").identity(), PostIdentity::Anonymous);
    assert!(!AuthorChoice::Pseudonym(" ".into()).is_complete());
    assert!(AuthorChoice::Yourself.is_complete());
    assert_eq!(pseudonym_handle("Publius Valerius"), "@publius_valerius");
    assert_eq!(pseudonym_handle("Мастер и Маргарита"), "@мастер_и_маргарита");
    assert_eq!(pseudonym_handle("!!!"), "@pen_name");
}

use super::diversity::measures::{mean_pairwise_distance, variety};
use super::diversity::stance::{reply_stance, ReplyStance};
use super::diversity::{Dimension, ShareLabel};
use super::timeline::Phase;
use super::*;
use crate::domain::insights::filter::DateRange;
use crate::domain::readability::Band;
use crate::domain::trait_option::TraitOption;
use crate::domain::traits::EpistemicStance;
use chrono::{Duration, TimeZone};
use std::time::{Duration as Elapsed, Instant};

fn now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 3, 10, 12, 0, 0).unwrap()
}

fn at(post: Post, days: i64) -> Post {
    Post { created_at: now() - Duration::days(days), ..post }
}

fn agent(id: &str, name: &str, stance: EpistemicStance) -> Profile {
    let mut profile = Profile::new_agent(id, name, &format!("@{id}"), "Thinker").with_network("n");
    profile.config.epistemic_stance = stance;
    profile
}

/// Two agents with different stances and the user: a root, a reply that pushes back, one
/// that agrees, an older post, and a recent burst by one agent.
fn input() -> LensInput {
    let me = Profile::new_primary("me", "Me", "@me");
    let socrates = agent("soc", "Socrates", EpistemicStance::FallibilistSkeptic);
    let spinoza = agent("spi", "Spinoza", EpistemicStance::HolisticMonist);
    let mut posts = vec![
        at(Post::create("root", "n", me.clone(), "Freedom is the recognition of necessity. #freedom #necessity").unwrap(), 20),
        at(Post::create_reply("no", "n", socrates.clone(), "I disagree: freedom is not mere recognition. #freedom", "root", None, None).unwrap(), 19),
        at(Post::create_reply("yes", "n", spinoza.clone(), "Exactly, well said. #necessity #freedom", "root", None, None).unwrap(), 18),
    ];
    posts.extend((0..6).map(|i| at(Post::create(format!("s{i}"), "n", socrates.clone(), &format!("Short question number {i}? #freedom")).unwrap(), i)));
    let mut doc = GroundingDocument::new("d1", "n", "Ethics", "Body");
    doc.created_at = now() - Duration::days(10);
    doc.record.subjects = vec!["ethics".into()];
    doc.record.reading = crate::domain::document::record::Reading::at(50);
    let review = vec![ReviewSource { network: "n".into(), text: "What is necessity?".into(), tags: vec!["necessity".into(), "ethics".into()], created_at: now() - Duration::days(3) }];
    let vectors = vec![("root".into(), vec![1.0, 0.0]), ("no".into(), vec![0.0, 1.0]), ("s0".into(), vec![0.0, 1.0]), ("s1".into(), vec![0.1, 1.0])];
    LensInput { network: "n".into(), posts, documents: vec![doc], profiles: vec![me, socrates, spinoza], subjects: vec![("ethics".into(), "Ethics".into())], review, vectors, language: Default::default(), now: now(), offset: None }
}

fn everything() -> InsightsFilter {
    InsightsFilter { range: DateRange::All, ..InsightsFilter::default() }
}

#[test]
fn the_timeline_reads_posts_documents_and_review_items() {
    let report = LensReport::compute(&input(), &everything());
    let freedom = report.timeline.lanes.iter().find(|l| l.key == "freedom").expect("freedom lane");
    assert_eq!(freedom.total, 9);
    let necessity = report.timeline.lanes.iter().find(|l| l.key == "necessity").expect("necessity lane");
    assert_eq!(necessity.total, 3, "root, the agreeing reply and the review item");
    assert!(report.timeline.mentions.iter().any(|m| m.kind == sources::SourceKind::Review));
    assert!(report.timeline.lanes.iter().any(|l| l.key == "ethics"), "the document's subject and title");
    assert_ne!(freedom.phase, Phase::Dormant);
}

#[test]
fn diversity_counts_authors_stances_and_reply_stances() {
    let report = LensReport::compute(&input(), &everything()).diversity;
    let authors = report.feed_shares(Dimension::Author).unwrap();
    assert_eq!((authors.shares[0].label.clone(), authors.shares[0].count), (ShareLabel::Name("Socrates".into()), 7));
    assert_eq!(authors.variety.sources, 3);
    let stances = report.feed_shares(Dimension::Stance).unwrap();
    assert_eq!(stances.variety.items, 8, "only agents' posts have a stance");
    assert_eq!((report.replies.agrees, report.replies.pushes_back, report.replies.neither), (1, 1, 0));
    assert_eq!(report.embedded, 4);
    assert!(report.spread.is_some());
    assert_eq!(report.documents_read, 1);
    assert_eq!(report.reading[0].shares[0].label, ShareLabel::Name("Ethics".into()));
}

#[test]
fn lately_names_the_busiest_voice_and_quieter_stances() {
    let lately = LensReport::compute(&input(), &everything()).diversity.lately.expect("recent posts");
    assert_eq!(lately.posts, 6);
    assert_eq!(lately.top_author, Some(("Socrates".into(), 1.0)));
    assert_eq!(lately.quieter, vec![EpistemicStance::HolisticMonist.label_key()]);
    assert_eq!(lately.agents[0].name, "Spinoza");
    assert_eq!(lately.further.first().map(|&i| LensReport::compute(&input(), &everything()).posts[i].id.as_str().to_string()), Some("root".into()));
}

#[test]
fn readability_scores_every_post_with_text() {
    let report = LensReport::compute(&input(), &everything()).readability;
    assert_eq!(report.scored.len(), 9);
    assert!(report.median_ease.is_some());
    let easiest = &report.scored[report.easiest[0]];
    let hardest = &report.scored[report.hardest[0]];
    assert!(easiest.score.ease >= hardest.score.ease);
    assert_eq!(report.authors[0].bands.iter().sum::<usize>(), 7, "Socrates writes most");
    assert_eq!(report.trend.iter().map(|p| p.posts).sum::<usize>(), 9);
    assert!(Band::ALL.len() == report.authors[0].bands.len());
}

#[test]
fn the_filter_applies_to_every_lens() {
    let week = InsightsFilter { range: DateRange::Week, ..InsightsFilter::default() };
    let report = LensReport::compute(&input(), &week);
    assert_eq!(report.posts.len(), 6);
    assert!(report.timeline.mentions.iter().all(|m| m.kind != sources::SourceKind::Document), "the document is older than a week");
    assert_eq!(report.readability.scored.len(), 6);
}

#[test]
fn variety_measures() {
    let even = variety(&[5, 5]);
    assert!((even.entropy - 1.0).abs() < 1e-9 && (even.evenness - 1.0).abs() < 1e-9 && (even.effective - 2.0).abs() < 1e-9);
    assert!((even.simpson - (1.0 - 40.0 / 90.0)).abs() < 1e-9);
    let single = variety(&[10, 0]);
    assert_eq!((single.sources, single.entropy, single.simpson, single.effective), (1, 0.0, 0.0, 1.0));
    assert_eq!(variety(&[]), Default::default());
    let (a, b) = ([1.0f32, 0.0], [0.0f32, 1.0]);
    assert!((mean_pairwise_distance(&[&a, &b]).unwrap() - 1.0).abs() < 1e-6);
    assert!(mean_pairwise_distance(&[&a]).is_none());
}

#[test]
fn reply_stances_read_the_surface() {
    assert_eq!(reply_stance("Free will exists.", "I disagree with that."), ReplyStance::PushesBack);
    assert_eq!(reply_stance("Free will exists.", "Free will does not exist."), ReplyStance::PushesBack);
    assert_eq!(reply_stance("Free will exists.", "Yes, but only in part."), ReplyStance::PushesBack);
    assert_eq!(reply_stance("Free will exists.", "Exactly, well said."), ReplyStance::Agrees);
    assert_eq!(reply_stance("La liberté existe.", "Tout à fait d'accord."), ReplyStance::Agrees);
    assert_eq!(reply_stance("La liberté existe.", "Je ne suis pas d'accord."), ReplyStance::PushesBack);
    assert_eq!(reply_stance("Free will exists.", "What about Spinoza?"), ReplyStance::Neither);
}

#[test]
fn review_items_contribute_their_own_words() {
    use crate::domain::review::item::{new_map_item, NewItemContext};
    use crate::domain::review::{ConnectionMap, ItemId};
    let map = ConnectionMap { title: "Spinoza and freedom".into(), ..Default::default() };
    let context = NewItemContext { network_id: "n".into(), created_at: now(), today: now().date_naive() };
    let item = ReviewItemTags::tagged(new_map_item(map, &context, ItemId::new("i1")), "ethics");
    let source = review_source(&item);
    assert_eq!((source.text.as_str(), source.tags.clone()), ("Spinoza and freedom", vec!["ethics".to_string()]));
}

/// Adds a tag to a review item.
trait ReviewItemTags {
    fn tagged(self, tag: &str) -> Self;
}

impl ReviewItemTags for crate::domain::review::ReviewItem {
    fn tagged(self, tag: &str) -> Self {
        Self { tags: vec![tag.to_string()], ..self }
    }
}

/// Every lens module and the shell's snapshot builder: none may read private journal data.
#[test]
fn lenses_never_touch_private_journal_data() {
    let files = [include_str!("mod.rs"), include_str!("sources.rs"), include_str!("diversity/mod.rs"), include_str!("diversity/lately.rs"), include_str!("readability.rs"), include_str!("timeline/mod.rs"), include_str!("../../../ui/features/shell/lens_host.rs")];
    assert!(files.iter().all(|source| !source.contains("journal::") && !source.contains(".journal")));
}

/// 5 000 posts by 40 authors over a year, each with hashtags and prose: every lens must stay
/// well inside the budget even in an unoptimised test build (a quadratic step would blow it).
#[test]
fn five_thousand_posts_fit_the_budget() {
    const POSTS: usize = 5_000;
    const BUDGET: Elapsed = Elapsed::from_secs(10);
    let authors: Vec<Profile> = (0..40).map(|i| agent(&format!("a{i}"), &format!("Agent {i}"), EpistemicStance::ALL[i % EpistemicStance::ALL.len()])).collect();
    let topics = ["freedom", "necessity", "ethics", "logic", "memory", "virtue", "justice", "language", "mind", "nature"];
    let posts: Vec<Post> = (0..POSTS).map(|i| {
        let text = format!("Thinking about {} and {} again, with Spinoza and Socrates in mind. #{} #{}", topics[i % 10], topics[(i / 3) % 10], topics[i % 10], topics[(i * 7) % 10]);
        let author = authors[i % authors.len()].clone();
        let post = if i % 3 == 1 { Post::create_reply(format!("p{i}"), "n", author, &text, format!("p{}", i - 1).as_str(), None, None) } else { Post::create(format!("p{i}"), "n", author, &text) };
        Post { created_at: now() - Duration::minutes((i * 105) as i64), ..post.unwrap() }
    }).collect();
    let vectors = (0..POSTS).step_by(2).map(|i| (format!("p{i}"), vec![(i % 7) as f32, (i % 5) as f32, 1.0])).collect();
    let input = LensInput { posts, profiles: authors, vectors, ..LensInput::empty("n".into(), now()) };
    let started = Instant::now();
    let report = LensReport::compute(&input, &everything());
    let elapsed = started.elapsed();
    assert_eq!(report.posts.len(), POSTS);
    assert!(report.timeline.lanes.len() >= topics.len());
    assert!(report.timeline.periods.count <= timeline::MAX_PERIODS);
    assert_eq!(report.readability.scored.len(), POSTS);
    assert!(elapsed < BUDGET, "lenses over 5 000 posts took {elapsed:?}");
}

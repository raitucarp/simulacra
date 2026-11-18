use super::{day, own_post, session_on};
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::gaps::evidence::{radar_concepts, MIN_POST_MENTIONS};
use crate::domain::learning::gaps::score::{dimension_scores, ranked, recency, saturate, RECENCY_HALF_LIFE_DAYS};
use crate::domain::learning::gaps::{collect_evidence, gap_report, strength, ConceptEvidence, GapDimension, GapEntry, GapInputs, ItemText, SourceText, DIMENSION_WEIGHTS};
use crate::domain::learning::goals::OwnPost;
use crate::domain::learning::tutor::{Assessment, Exchange, SocraticMove, TutorQuestion, TutorSource, Verdict};

fn inputs<'a>(posts: &'a [OwnPost], sources: &'a [SourceText], items: &'a [ItemText], sessions: &'a [crate::domain::learning::TutorSession]) -> GapInputs<'a> {
    GapInputs { own_posts: posts, sources, items, sessions, pinned: &[], only: None, today: day(10) }
}

fn evidence(own_posts: usize, sources: usize, retention: Option<f64>, mastery: Option<BloomLevel>, degree: usize, last: Option<i64>) -> ConceptEvidence {
    ConceptEvidence { concept: "c".into(), mentions: 1, own_posts, sources, items: 0, retention, mastery, degree, last_touch: last.map(day) }
}

fn entry(concept: &str, strength: f64, mentions: usize) -> GapEntry {
    GapEntry { concept: concept.into(), mentions, scores: [strength; 6], strength }
}

#[test]
fn the_weights_sum_to_one() {
    assert!((DIMENSION_WEIGHTS.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    assert_eq!(GapDimension::ALL.len(), DIMENSION_WEIGHTS.len());
}

#[test]
fn each_dimension_scores_as_documented() {
    assert_eq!(saturate(0, 2.0), 0.0);
    assert_eq!(saturate(2, 2.0), 0.5);
    assert_eq!(recency(None, day(10)), 0.0);
    assert!((recency(Some(day(10)), day(10)) - 1.0).abs() < 1e-9);
    assert!((recency(Some(day(10 - RECENCY_HALF_LIFE_DAYS as i64)), day(10)) - 0.5).abs() < 1e-9);
    let scores = dimension_scores(&evidence(2, 1, Some(0.8), Some(BloomLevel::Create), 3, Some(10)), day(10));
    assert_eq!(scores, [0.5, 0.5, 0.8, 1.0, 0.5, 1.0]);
    assert_eq!(dimension_scores(&evidence(0, 0, None, None, 0, None), day(10)), [0.0; 6]);
    assert!((strength(&[1.0; 6]) - 1.0).abs() < 1e-9);
}

#[test]
fn the_weakest_concepts_come_first_and_ties_favour_the_more_mentioned() {
    let order: Vec<String> = ranked(vec![entry("strong", 0.9, 1), entry("weak", 0.1, 1), entry("tied b", 0.4, 1), entry("tied a", 0.4, 5)]).into_iter().map(|e| e.concept).collect();
    assert_eq!(order, ["weak", "tied a", "tied b", "strong"]);
    let lopsided = GapEntry { concept: "x".into(), mentions: 1, scores: [0.9, 0.1, 0.5, 0.5, 0.5, 0.5], strength: 0.5 };
    assert_eq!(lopsided.weakest(), GapDimension::Sources);
    assert!((lopsided.gap() - 0.5).abs() < 1e-9);
}

#[test]
fn concepts_need_two_posts_unless_marked_tagged_or_pinned() {
    let posts = [own_post("a", "Thinking about causation and structure.", 1), own_post("b", "More causation here.", 2), own_post("c", "On #ethics", 3), own_post("d", "Something about structure.", 4)];
    let items = [ItemText { text: "Q".into(), tags: vec!["Modes".into()], retrievability: None, last_review: None }];
    let mut input = inputs(&posts, &[], &items, &[]);
    let pinned = ["Attributes".to_string()];
    input.pinned = &pinned;
    let post_concepts: Vec<Vec<String>> = posts.iter().map(|p| crate::domain::learning::concept::concepts_of(&p.text)).collect();
    let concepts = radar_concepts(&input, &post_concepts);
    assert_eq!(concepts[0], "attributes", "pinned concepts lead");
    for wanted in ["causation", "structure", "ethics", "modes"] {
        assert!(concepts.contains(&wanted.to_string()), "{wanted} in {concepts:?}");
    }
    assert!(MIN_POST_MENTIONS == 2 && !concepts.contains(&"thinking".to_string()));
    let only = ["Ethics".to_string()];
    input.only = Some(&only);
    assert_eq!(radar_concepts(&input, &post_concepts), vec!["ethics".to_string()]);
}

#[test]
fn evidence_gathers_posts_sources_items_sessions_and_neighbours() {
    let posts = [own_post("a", "#necessity and #freedom", 4), own_post("b", "#necessity rules", 8)];
    let sources = [SourceText { title: "Ethics".into(), text: "On necessity and the mind".into() }];
    let items = [ItemText { text: "What is necessity?".into(), tags: vec![], retrievability: Some(0.6), last_review: Some(day(9)) }];
    let mut session = session_on("p");
    session.source = TutorSource::Concept { name: "Necessity".into() };
    session.title = "Necessity".into();
    session.exchanges.push(Exchange {
        question: TutorQuestion { text: "Q?".into(), level: BloomLevel::Analyse, kind: SocraticMove::Analyse },
        answer: Some("a".into()),
        assessment: Some(Assessment { reached: None, verdict: Verdict::Solid, right: vec![], misconceptions: vec![] }),
    });
    let sessions = [session];
    let all = collect_evidence(&inputs(&posts, &sources, &items, &sessions));
    let necessity = all.iter().find(|e| e.concept == "necessity").expect("necessity");
    assert_eq!((necessity.own_posts, necessity.sources, necessity.items), (2, 1, 1));
    assert_eq!(necessity.retention, Some(0.6));
    assert_eq!(necessity.mastery, Some(BloomLevel::Analyse));
    assert_eq!(necessity.degree, 1, "it shares a post with freedom");
    assert_eq!(necessity.last_touch, Some(day(9)));
    let report = gap_report(&inputs(&posts, &sources, &items, &sessions));
    assert_eq!(report.entries.last().map(|e| e.concept.as_str()), Some("necessity"), "the best held concept comes last");
    assert!(report.average.iter().all(|v| (0.0..=1.0).contains(v)));
    assert_eq!(report.weakest(1).len(), 1);
}

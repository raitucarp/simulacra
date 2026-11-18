use super::{agent, now, setup};
use crate::domain::arena::debate::{Debate, DebateFormat, Side};
use crate::domain::arena::lab::summary::{summary_text, SummaryWords};
use crate::domain::arena::lab::{CellAnswer, Experiment, Scenario, Stance, Variable};
use crate::domain::arena::publish::{build_thread, debate_parts, split_parts, ThreadPart};
use crate::domain::arena::ArenaId;
use crate::domain::{NetworkId, PostId};

#[test]
fn a_debate_becomes_one_continuous_thread_by_its_speakers() {
    let setup = setup(DebateFormat::Oxford, &[("a", Side::For), ("b", Side::Against)], 1);
    let long = "word ".repeat(80);
    let debate = Debate::new(ArenaId::new("d"), setup, now()).with_turn(&long, now()).with_turn("Short reply.", now());
    let root = ThreadPart { author: agent("me"), text: "Motion: ban cars".into() };
    let parts = split_parts(&debate_parts(&debate, root, None), 120);
    assert!(parts.len() > 3, "the long turn continues in follow-ups");
    assert!(parts[1..parts.len() - 1].iter().all(|part| part.author == agent("a")));
    let mut next = 0;
    let posts = build_thread(&parts, &NetworkId::new("n"), now(), || {
        next += 1;
        PostId::new(format!("p{next}"))
    })
    .expect("posts");
    assert_eq!(posts[0].reply_to_id, None);
    assert!(posts.windows(2).all(|pair| pair[1].reply_to_id.as_ref() == Some(&pair[0].id) && pair[0].replies_count == 1));
    assert!(posts.windows(2).all(|pair| pair[0].created_at < pair[1].created_at));
    assert_eq!(posts.last().map(|post| post.author_id.clone()), Some(agent("b")));
}

#[test]
fn the_summary_counts_stances_and_names_the_pivot() {
    let scenario = Scenario {
        title: "T".into(),
        premise: "P".into(),
        variables: vec![Variable { name: "Means".into(), values: vec!["lever".into(), "push".into()] }],
        question: "Act?".into(),
    };
    let mut experiment = Experiment::new(ArenaId::new("x"), scenario, vec![agent("a")], None, now());
    for (cell, stance) in [(0, Stance::Yes), (1, Stance::No)] {
        experiment = experiment.with_answer(CellAnswer { agent: agent("a"), cell, stance, verdict: String::new(), reasoning: String::new() }, now());
    }
    let words = SummaryWords { heading: "Experiment: T".into(), stances: vec!["yes".into(), "no".into(), "unsure".into()], turns_on: "Turns on {variable}".into(), steady: "Steady".into() };
    let text = summary_text(&experiment, &words, &|id| id.to_string());
    assert!(text.starts_with("Experiment: T\nAct?"));
    assert!(text.contains("a: yes 1 · no 1"));
    assert!(text.ends_with("Turns on Means"));
}

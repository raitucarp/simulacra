use super::{agent, name, now, setup};
use crate::domain::arena::debate::{claim_standings, effective_judge, judge_prompt, parse_verdict, turn_posts, Criterion, Debate, DebateFormat, Judge, Side};
use crate::domain::arena::ArenaId;
use crate::domain::thinking::argument::{ArgEdge, ArgNode, ArgumentMap, EdgeKind, NodeRole, Standing};

fn spoken() -> Debate {
    let setup = setup(DebateFormat::Oxford, &[("a", Side::For), ("b", Side::Against)], 1);
    Debate::new(ArenaId::new("d"), setup, now()).with_turn("Clean air saves lives.", now()).with_turn("Deliveries need cars.", now())
}

#[test]
fn the_verdict_reads_labels_clamps_marks_and_drops_strangers() {
    let debate = spoken();
    let raw = r#"Here: ```json
{"scores":[{"debater":"D1","evidence":"7","reasoning":12,"responsiveness":6,"charity":5,"note":"Clear."},
{"debater":"d2","evidence":4,"reasoning":5,"responsiveness":8,"charity":9},{"debater":"D9","evidence":10}],"summary":"Close."}```"#;
    let verdict = parse_verdict(raw, &debate.setup, Judge::GlobalModel).expect("verdict");
    assert_eq!(verdict.scores.len(), 2);
    assert_eq!(verdict.scores[0].agent, agent("a"));
    assert_eq!(verdict.scores[0].mark(Criterion::Reasoning), 10, "clamped to the scale");
    assert_eq!(verdict.scores[0].mark(Criterion::Evidence), 7, "numbers may come as text");
    assert_eq!(verdict.scores[1].total(), 26);
    assert_eq!(verdict.summary, "Close.");
    assert_eq!(verdict.leading_side(&debate.setup), Some(Side::For), "28 against 26");
    assert!(parse_verdict("no json", &debate.setup, Judge::GlobalModel).is_none());
}

#[test]
fn the_judge_prompt_names_debaters_and_the_rubric() {
    let debate = spoken();
    let prompt = judge_prompt(&debate.setup, &debate.turns, &name, "French");
    assert!(prompt.user.contains("D1 = Name of a (for)"));
    assert!(prompt.user.contains("charity"));
    assert!(prompt.user.contains("in French"));
    assert!(prompt.system.contains("impartial"));
}

#[test]
fn a_judge_who_debates_gives_way_to_the_global_model() {
    let mut setup = spoken().setup;
    setup.judge = Judge::Agent(agent("a"));
    assert_eq!(effective_judge(&setup), Judge::GlobalModel);
    setup.judge = Judge::Agent(agent("j"));
    assert_eq!(effective_judge(&setup), Judge::Agent(agent("j")));
}

#[test]
fn claims_stand_or_fall_by_the_grounded_labelling() {
    let debate = spoken();
    let posts = turn_posts(&debate, &name);
    assert_eq!(posts[1].id, debate.turn_ref(1));
    let node = |id: &str, role: NodeRole, text: &str, turn: usize| ArgNode { id: id.into(), role, text: text.into(), source: debate.turn_ref(turn) };
    let map = ArgumentMap {
        root: debate.turn_ref(0),
        nodes: vec![node("n1", NodeRole::Claim, "Ban cars", 0), node("n2", NodeRole::Rebuttal, "Deliveries", 1), node("n3", NodeRole::Claim, "Keep cars", 1)],
        edges: vec![ArgEdge { from: "n2".into(), to: "n1".into(), kind: EdgeKind::Attack }],
    };
    let claims = claim_standings(&debate, &map);
    assert_eq!(claims.len(), 2);
    assert_eq!((claims[0].text.as_str(), claims[0].standing), ("Keep cars", Standing::Accepted));
    assert_eq!(claims[0].speaker, Some(agent("b")));
    assert_eq!((claims[1].text.as_str(), claims[1].standing), ("Ban cars", Standing::Rejected));
}

//! Tier A, thinking tools: post actions read by their domain parser, the argument map, the
//! tutor (opening then a turn) and the arena (a debate turn through the engine, a judge).

use super::harness::{ensure, pass, quote, run, Key, Live, Model, Outcome, Tier};
use simulacra::agent::arena::{run_call_blocking, EngineCall};
use simulacra::agent::learning::{run_tutor_blocking, TutorReply};
use simulacra::agent::post_actions::{run_action_blocking, ActionJob, CancelToken};
use simulacra::agent::runtime::EnginePlan;
use simulacra::domain::arena::debate::{judge_prompt, parse_verdict, turn_prompt, Debate, DebateFormat, DebateSetup, Debater, Judge, Phase, Side};
use simulacra::domain::arena::ArenaId;
use simulacra::domain::learning::tutor::machine::SessionStart;
use simulacra::domain::learning::tutor::prompt::TutorRequest;
use simulacra::domain::learning::{SessionId, TutorSession, TutorSource};
use simulacra::domain::post_actions::{build_prompt, parse_output, ActionContext, ActionKind, ActionPrompt, ActionRequest, AuthorContext, OutputBlock, RequestPolicy, ThreadLine};
use simulacra::domain::thinking::argument::{extraction_prompt, parse_extraction, ThreadPost};
use simulacra::domain::{NetworkId, PostId, ProfileId};
use simulacra::i18n::Language;
use std::time::Duration;

const LANGUAGE: &str = "English";

/// The runner's policy with a generous timeout and one retry.
fn policy() -> RequestPolicy {
    RequestPolicy { timeout: Duration::from_secs(120), max_retries: 1 }
}

/// An action job for `prompt` whose answer budget is at least `answer_tokens` (as the app does).
fn action_job(live: &Live, prompt: ActionPrompt, answer_tokens: u32) -> ActionJob {
    let mut settings = live.settings();
    settings.max_tokens = settings.max_tokens.max(answer_tokens);
    ActionJob { client: simulacra::agent::LlmClient::new(settings), prompt, policy: policy(), cancel: CancelToken::new() }
}

fn context() -> ActionContext {
    ActionContext {
        thought: ThreadLine::new("Baruch", "baruch", "Freedom is understanding necessity; nobody chooses their causes.", 0),
        author: AuthorContext { bio: "Rationalist agent; grinds lenses.".into(), is_agent: true },
        ancestors: vec![ThreadLine::new("Arya", "arya", "Is anyone free if every cause is fixed?", 0)],
        replies: vec![ThreadLine::new("Ada", "ada", "Then praise and blame lose their point.", 1)],
        reader_language: Language::English,
    }
}

/// Runs `kind` and checks that its parser finds what the action promises.
fn action_case(live: &Live, kind: ActionKind) -> Outcome {
    let prompt = build_prompt(&ActionRequest::new(kind, Language::English, ""), &context());
    let text = run_action_blocking(&action_job(live, prompt, kind.answer_tokens())).map_err(|f| format!("{:?}: {}", f.kind, f.detail))?;
    let output = parse_output(kind, &text);
    ensure(!output.is_empty(), || format!("nothing parsed from {}", quote(&text)))?;
    let relations = output.blocks.iter().filter(|b| matches!(b, OutputBlock::Relation(_))).count();
    let scores = output.blocks.iter().filter(|b| matches!(b, OutputBlock::Score { .. })).count();
    match kind {
        ActionKind::MapConcepts => ensure(relations > 0, || format!("no concept relation parsed from {}", quote(&text)))?,
        ActionKind::ClaimConfidence => ensure(scores > 0, || format!("no scored claim parsed from {}", quote(&text)))?,
        _ => {}
    }
    pass(format!("{} blocks ({relations} relations, {scores} scores)", output.blocks.len()))
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a10_post_action_steelman() {
    run(Tier::A, "post action: steelman", Model::Text, Key::Needed, |live| action_case(live, ActionKind::Steelman));
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a11_post_action_map_concepts() {
    run(Tier::A, "post action: map concepts (relations)", Model::Text, Key::Needed, |live| action_case(live, ActionKind::MapConcepts));
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a12_post_action_claim_confidence() {
    run(Tier::A, "post action: claim confidence (scores)", Model::Text, Key::Needed, |live| action_case(live, ActionKind::ClaimConfidence));
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a13_argument_map_extraction() {
    run(Tier::A, "argument map (JSON graph)", Model::Text, Key::Needed, |live| {
        let post = |id: &str, author: &str, text: &str| ThreadPost { id: PostId::new(id), author: author.into(), text: text.into() };
        let posts = vec![
            post("p1", "Arya", "Cities should ban private cars because cars kill pedestrians."),
            post("p2", "Ada", "Buses kill pedestrians too, so the reason proves too much."),
            post("p3", "Arya", "Per passenger mile buses are far safer, so the reason stands."),
        ];
        let text = run_action_blocking(&action_job(live, extraction_prompt(&posts, LANGUAGE), simulacra::domain::thinking::argument::EXTRACT_ANSWER_TOKENS))
            .map_err(|f| format!("{:?}: {}", f.kind, f.detail))?;
        let extraction = parse_extraction(&text, &posts).ok_or_else(|| format!("no argument map parsed from {}", quote(&text)))?;
        pass(format!("{} nodes, {} edges ({} nodes dropped)", extraction.map.nodes.len(), extraction.map.edges.len(), extraction.dropped_nodes))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a14_tutor_opening_then_turn() {
    run(Tier::A, "tutor: opening question then a turn", Model::Text, Key::Needed, |live| {
        let now = jiff::Timestamp::now;
        let start = SessionStart { network_id: NetworkId::new("net_live"), source: TutorSource::Concept { name: "necessity".into() }, title: "Necessity".into(), material: "Necessity: what could not be otherwise.".into() };
        let session = TutorSession::start(SessionId("tutor_live".into()), start, now());
        let tutor = |request: TutorRequest, session: &TutorSession| {
            run_tutor_blocking(action_job(live, request.prompt(session, LANGUAGE), request.answer_tokens()), request, None).map_err(|f| format!("{f:?}"))
        };
        let TutorReply::Turn(opening) = tutor(TutorRequest::Opening, &session)? else { return Err("the opening was not a question".into()) };
        let answered = session.with_opening(&opening.question, now()).with_answer("Something that must be so, like 2 + 2 = 4.", now());
        let TutorReply::Turn(turn) = tutor(TutorRequest::Turn, &answered)? else { return Err("the turn was not a question".into()) };
        ensure(turn.assessment.is_some(), || "the turn has no assessment".to_string())?;
        pass(format!("asked {}, then {}", quote(&opening.question), quote(&turn.question)))
    });
}

fn debate_setup() -> DebateSetup {
    let debater = |id: &str, side| Debater { agent: ProfileId::new(id), side };
    DebateSetup {
        motion: "Cities should ban private cars".into(),
        source_post: None,
        debaters: vec![debater("agent_ada", Side::For), debater("agent_baruch", Side::Against)],
        format: DebateFormat::Oxford,
        rounds: 1,
        moderator: None,
        judge: Judge::GlobalModel,
    }
}

fn name(id: &ProfileId) -> String {
    if id.as_str() == "agent_ada" { "Ada".into() } else { "Baruch".into() }
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a15_arena_turn_through_the_engine() {
    run(Tier::A, "arena: debate turn (engine call)", Model::Text, Key::Needed, |live| {
        let setup = debate_setup();
        let debate = Debate::new(ArenaId::new("arena_live"), setup.clone(), jiff::Timestamp::now());
        let Phase::Speaking(slot) = debate.phase() else { return Err("the debate has no first slot".into()) };
        let prompt = ActionPrompt { system: "You are Ada, a careful urbanist who debates in good faith.".into(), user: turn_prompt(&setup, &debate.turns, &slot, &name, LANGUAGE) };
        let call = EngineCall::new(EnginePlan::from_client(live.client()), prompt, policy(), CancelToken::new());
        let text = run_call_blocking(&call).map_err(|f| format!("{:?}: {}", f.kind, f.detail))?;
        pass(format!("{} words: {}", text.split_whitespace().count(), quote(&text)))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a16_arena_judge_verdict() {
    run(Tier::A, "arena: judge verdict (JSON scores)", Model::Text, Key::Needed, |live| {
        let setup = debate_setup();
        let now = jiff::Timestamp::now();
        let debate = Debate::new(ArenaId::new("arena_live"), setup.clone(), now)
            .with_turn("Private cars kill pedestrians and fill streets; buses and bikes move more people safely.", now)
            .with_turn("A ban punishes the disabled and rural commuters; price congestion instead of banning.", now);
        let prompt = judge_prompt(&setup, &debate.turns, &name, LANGUAGE);
        let text = run_action_blocking(&action_job(live, prompt, simulacra::domain::arena::debate::JUDGE_ANSWER_TOKENS)).map_err(|f| format!("{:?}: {}", f.kind, f.detail))?;
        let verdict = parse_verdict(&text, &setup, Judge::GlobalModel).ok_or_else(|| format!("no verdict parsed from {}", quote(&text)))?;
        ensure(verdict.scores.len() == 2, || format!("{} scores for 2 debaters", verdict.scores.len()))?;
        pass(format!("2 scores, summary {}", quote(&verdict.summary)))
    });
}

//! Cards for the post menu tests: the user's own post, someone else's, and one per state the
//! old menu reacted to (in review, archived, a task, planted, in a series, corrected, …).

mod inventory;
mod structure;

use super::PostMenuSource;
use crate::domain::post::garden::Maturity;
use crate::domain::post::open_question::QuestionState;
use crate::domain::PostId;
use crate::ui::features::data::{DataControls, DataSource, IdeaSource, PostFacts};
use crate::ui::features::post::controls::{LearnControls, LearnStatus, PostAction};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn noop() -> PostAction {
    Rc::new(|_, _| {})
}

fn pick() -> crate::ui::features::post::controls::PickPostAction {
    Rc::new(|_, _, _| {})
}

fn handler() -> crate::ui::menus::MenuHandler {
    Rc::new(|_, _| {})
}

fn ideas() -> IdeaSource {
    let agent = (crate::domain::ProfileId::new("agent_1"), "Hypatia".to_string());
    IdeaSource { agents: vec![agent], series: vec!["On virtue".to_string()], study_language: "Español".to_string(), ..IdeaSource::default() }
}

/// The data controls of post `p1`, written by the user when `own`.
pub(super) fn data(own: bool) -> DataControls {
    let collections = Rc::new(vec![(crate::domain::document::CollectionId::new("c1"), "Reading list".to_string())]);
    let source = DataSource { revisions: Rc::new(HashMap::new()), bookmarks: Rc::new(HashSet::new()), collections, memberships: Rc::new(HashMap::new()), history_open: false, depth: Default::default(), ideas: Rc::new(ideas()), on_event: Rc::new(|_, _, _| {}) };
    source.controls(&PostId::new("p1"), own)
}

/// A card whose data controls `tune` adjusts.
pub(super) fn card(own: bool, tune: impl FnOnce(&mut DataControls)) -> PostMenuSource {
    let mut controls = data(own);
    tune(&mut controls);
    PostMenuSource {
        copies: ["An idea worth keeping.".to_string(), "simulacra://n1/post/p1".to_string(), "((p1@1.0.0))".to_string()],
        on_reply: noop(),
        on_quote: noop(),
        on_find_similar: noop(),
        on_post_action: (!controls.locked).then(pick),
        learn: LearnControls::inert(),
        set_credence: own.then(handler),
        data: Some(controls),
        on_delete: own.then(noop),
    }
}

pub(super) fn own() -> PostMenuSource {
    card(true, |_| {})
}

pub(super) fn others() -> PostMenuSource {
    card(false, |_| {})
}

pub(super) fn learning() -> PostMenuSource {
    PostMenuSource { learn: LearnControls { status: Some(LearnStatus::Due), on_learn: Rc::new(|_, _, _| {}) }, ..own() }
}

pub(super) fn archived() -> PostMenuSource {
    card(true, |c| c.lifecycle.status = crate::domain::post::PostStatus::Archived)
}

pub(super) fn task() -> PostMenuSource {
    card(true, |c| {
        c.symbols.entry = Some(crate::domain::symbols::EntrySymbol::Task);
        c.symbols.state = Some(crate::domain::symbols::EntryState::Open);
    })
}

pub(super) fn planted() -> PostMenuSource {
    card(true, |c| c.facts.garden = Some(Maturity::Budding))
}

pub(super) fn in_series() -> PostMenuSource {
    card(true, |c| c.facts = PostFacts { series: Some("On virtue".to_string()), ..PostFacts::default() })
}

pub(super) fn repinnable() -> PostMenuSource {
    card(true, |c| c.facts.repinnable = true)
}

pub(super) fn edited() -> PostMenuSource {
    card(false, |c| c.revisions = 2)
}

pub(super) fn voice() -> PostMenuSource {
    card(true, |c| c.facts.transcribable = true)
}

pub(super) fn corrected() -> PostMenuSource {
    card(true, |c| c.facts.correction = Some(crate::domain::post::correction::CorrectionKind::Corrected))
}

pub(super) fn translated() -> PostMenuSource {
    card(false, |c| {
        c.facts.translation = true;
        c.ideas = Rc::new(IdeaSource { translated: vec![PostId::new("p1")], ..ideas() });
    })
}

pub(super) fn answering() -> PostMenuSource {
    card(false, |c| c.facts.answers = Some(PostId::new("q1")))
}

pub(super) fn defining() -> PostMenuSource {
    card(true, |c| c.facts.defines = Some("eudaimonia".to_string()))
}

pub(super) fn question(state: QuestionState) -> PostMenuSource {
    card(true, |c| c.facts.question = Some(state))
}

pub(super) fn changed_mind() -> PostMenuSource {
    card(true, |c| c.facts.changed_mind = true)
}

pub(super) fn locked() -> PostMenuSource {
    card(true, |c| c.locked = true)
}

/// Every card above, for the rules that hold on all of them.
pub(super) fn every_card() -> Vec<(&'static str, PostMenuSource)> {
    vec![
        ("own", own()), ("others", others()), ("learning", learning()), ("archived", archived()), ("task", task()), ("planted", planted()),
        ("in series", in_series()), ("repinnable", repinnable()), ("edited", edited()), ("voice", voice()), ("corrected", corrected()),
        ("translated", translated()), ("answering", answering()), ("defining", defining()), ("open question", question(QuestionState::Open)),
        ("answered", question(QuestionState::Answered)), ("changed mind", changed_mind()), ("locked", locked()),
    ]
}

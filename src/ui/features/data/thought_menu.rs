//! N16–N20: a post's menu entries: the lens fan and showing it in the language the user is
//! learning (any post); defining a term in it, making it an open question and "Changed my
//! mind…" (the user's own); "This answers my question" on a reply to the user's open
//! question; sending words of a translation to review. None on a locked post.

use super::events::DataEvent;
use super::glossary_sheet::GlossaryEvent;
use super::lens_sheet::LensEvent;
use super::mind_sheet::MindEvent;
use super::post_menu::{item, DataControls};
use super::questions_sheet::QuestionsEvent;
use super::thought_events::ThoughtEvent;
use super::translation_sheet::TranslateEvent;
use crate::domain::post::open_question::QuestionState;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuItem, MenuText};
use rust_i18n::t;

fn thought(event: ThoughtEvent) -> DataEvent {
    DataEvent::Thought(event)
}

/// Whether the thinking items apply at all (never on a locked post).
fn open(controls: &DataControls) -> bool {
    !controls.locked
}

/// Whether the items of the user's own thinking apply.
fn own(controls: &DataControls) -> bool {
    controls.own && open(controls)
}

/// N16: "Lens fan…".
pub(super) fn lens_item(controls: &DataControls) -> Option<MenuItem> {
    open(controls).then(|| item(ids::LENS_FAN, MenuText::key("posts.lens.menu"), LucideIcon::Layers, thought(ThoughtEvent::Lens(LensEvent::Open(controls.post_id.clone()))), &controls.on_event))
}

/// N20: show the post in the language the user is learning, or as written again.
pub(super) fn translation_item(controls: &DataControls) -> Option<MenuItem> {
    if !open(controls) {
        return None;
    }
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    Some(match controls.ideas.translated.contains(post) {
        true => item(ids::TRANSLATE_ORIGINAL, MenuText::key("posts.translation.original"), LucideIcon::RotateCcw, thought(ThoughtEvent::Translate(TranslateEvent::Original(post.clone()))), on_event),
        false => {
            let label = MenuText::keyed("posts.translation.show", t!("posts.translation.show", language = controls.ideas.study_language.clone()).to_string());
            item(ids::TRANSLATE_SHOW, label, LucideIcon::Languages, thought(ThoughtEvent::Translate(TranslateEvent::Show(post.clone()))), on_event)
        }
    })
}

/// N20: "Send words to review…" once the post keeps a translation.
pub(super) fn words_item(controls: &DataControls) -> Option<MenuItem> {
    (open(controls) && controls.facts.translation).then(|| item(ids::WORDS_TO_REVIEW, MenuText::key("posts.translation.words_menu"), LucideIcon::GraduationCap, thought(ThoughtEvent::Translate(TranslateEvent::Words(controls.post_id.clone()))), &controls.on_event))
}

/// N18: "This answers my question" on a reply to the user's open question.
pub(super) fn answers_item(controls: &DataControls) -> Option<MenuItem> {
    let question = controls.facts.answers.as_ref().filter(|_| open(controls))?;
    let event = thought(ThoughtEvent::Questions(QuestionsEvent::Resolve(question.clone(), Some(controls.post_id.clone()))));
    Some(item(ids::QUESTION_ANSWERS, MenuText::key("posts.open_question.menu_answers"), LucideIcon::CircleCheck, event, &controls.on_event))
}

/// N17: define a term in the user's post, or stop defining it.
pub(super) fn glossary_items(controls: &DataControls) -> Vec<MenuItem> {
    if !own(controls) {
        return Vec::new();
    }
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let define = item(ids::DEFINE_TERM, MenuText::key("posts.glossary.menu"), LucideIcon::BookOpen, thought(ThoughtEvent::Glossary(GlossaryEvent::Open(post.clone()))), on_event);
    let undefine = controls.facts.defines.as_ref().map(|term| {
        let label = MenuText::keyed("posts.glossary.undefine", t!("posts.glossary.undefine", term = term.clone()).to_string());
        item(ids::UNDEFINE_TERM, label, LucideIcon::X, thought(ThoughtEvent::Glossary(GlossaryEvent::Undefine(post.clone()))), on_event)
    });
    std::iter::once(define).chain(undefine).collect()
}

/// N18: make the user's post an open question, mark it answered, reopen it, or make it a
/// thought again (a feeling is never a question).
pub(super) fn question_items(controls: &DataControls) -> Vec<MenuItem> {
    if !own(controls) {
        return Vec::new();
    }
    let post = &controls.post_id;
    let entry = |id: &'static str, key: &'static str, icon: LucideIcon, event: QuestionsEvent| item(id, MenuText::key(key), icon, thought(ThoughtEvent::Questions(event)), &controls.on_event);
    match controls.facts.question {
        None if !controls.facts.feeling => vec![entry(ids::QUESTION_ASK, "posts.open_question.menu_ask", LucideIcon::HelpCircle, QuestionsEvent::SetQuestion(post.clone(), true))],
        None => Vec::new(),
        Some(QuestionState::Open) => vec![
            entry(ids::QUESTION_RESOLVE, "posts.open_question.menu_resolve", LucideIcon::CircleCheck, QuestionsEvent::Resolve(post.clone(), None)),
            entry(ids::QUESTION_UNASK, "posts.open_question.menu_unask", LucideIcon::X, QuestionsEvent::SetQuestion(post.clone(), false)),
        ],
        Some(QuestionState::Answered) => vec![entry(ids::QUESTION_REOPEN, "posts.open_question.menu_reopen", LucideIcon::RotateCcw, QuestionsEvent::Reopen(post.clone()))],
    }
}

/// N19: mark the user's post as one that changed their mind, or take the mark back.
pub(super) fn mind_items(controls: &DataControls) -> Vec<MenuItem> {
    if !own(controls) {
        return Vec::new();
    }
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let mark = item(ids::CHANGED_MIND, MenuText::key("posts.mind.menu"), LucideIcon::RefreshCw, thought(ThoughtEvent::Mind(MindEvent::Open(post.clone()))), on_event);
    let unmark = controls.facts.changed_mind.then(|| item(ids::UNMARK_MIND, MenuText::key("posts.mind.menu_unmark"), LucideIcon::X, thought(ThoughtEvent::Mind(MindEvent::Unmark(post.clone()))), on_event));
    std::iter::once(mark).chain(unmark).collect()
}

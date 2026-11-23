//! N16–N20: a post's "…" menu items: the lens fan (any post); defining a term in it or taking
//! the definition back, and making it a question, marking it answered or reopening it (the
//! user's own); and "This answers my question" on a reply to the user's open question; "Changed my mind…" (the user's
//! own). None on a locked post.

use super::events::DataEvent;
use super::glossary_sheet::GlossaryEvent;
use super::lens_sheet::LensEvent;
use super::mind_sheet::MindEvent;
use super::post_menu::{item, DataControls};
use super::questions_sheet::QuestionsEvent;
use super::text::tr;
use super::thought_events::ThoughtEvent;
use super::translation_sheet::TranslateEvent;
use crate::domain::post::open_question::QuestionState;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::*;
use rust_i18n::t;

fn thought(event: ThoughtEvent) -> DataEvent {
    DataEvent::Thought(event)
}

/// N17: define a term in the user's post, or stop defining it.
fn glossary_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let menu = menu.item(item(tr("posts.glossary.menu"), LucideIcon::BookOpen, thought(ThoughtEvent::Glossary(GlossaryEvent::Open(post.clone()))), on_event));
    match &controls.facts.defines {
        Some(term) => menu.item(item(t!("posts.glossary.undefine", term = term.clone()).to_string(), LucideIcon::X, thought(ThoughtEvent::Glossary(GlossaryEvent::Undefine(post.clone()))), on_event)),
        None => menu,
    }
}

/// N18: make the user's post a question, mark it answered, reopen it, or make it a thought again.
fn question_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let entry = |key: &str, icon: LucideIcon, event: QuestionsEvent| item(tr(key), icon, thought(ThoughtEvent::Questions(event)), on_event);
    match controls.facts.question {
        None if !controls.facts.feeling => menu.item(entry("posts.open_question.menu_ask", LucideIcon::HelpCircle, QuestionsEvent::SetQuestion(post.clone(), true))),
        None => menu,
        Some(QuestionState::Open) => menu.item(entry("posts.open_question.menu_resolve", LucideIcon::CircleCheck, QuestionsEvent::Resolve(post.clone(), None))).item(entry("posts.open_question.menu_unask", LucideIcon::X, QuestionsEvent::SetQuestion(post.clone(), false))),
        Some(QuestionState::Answered) => menu.item(entry("posts.open_question.menu_reopen", LucideIcon::RotateCcw, QuestionsEvent::Reopen(post.clone()))),
    }
}

/// N19: mark the user's post as one that changed their mind, or take the mark back.
fn mind_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let menu = menu.item(item(tr("posts.mind.menu"), LucideIcon::RefreshCw, thought(ThoughtEvent::Mind(MindEvent::Open(post.clone()))), on_event));
    if !controls.facts.changed_mind {
        return menu;
    }
    menu.item(item(tr("posts.mind.menu_unmark"), LucideIcon::X, thought(ThoughtEvent::Mind(MindEvent::Unmark(post.clone()))), on_event))
}

/// N20: show the post in the language the user is learning (or as written again), and send
/// words of its translation to review.
fn translation_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    let entry = |label: String, icon: LucideIcon, event: TranslateEvent| item(label, icon, thought(ThoughtEvent::Translate(event)), on_event);
    let menu = match controls.ideas.translated.contains(post) {
        true => menu.item(entry(tr("posts.translation.original"), LucideIcon::RotateCcw, TranslateEvent::Original(post.clone()))),
        false => menu.item(entry(t!("posts.translation.show", language = controls.ideas.study_language.clone()).to_string(), LucideIcon::Languages, TranslateEvent::Show(post.clone()))),
    };
    if !controls.facts.translation {
        return menu;
    }
    menu.item(entry(tr("posts.translation.words_menu"), LucideIcon::GraduationCap, TranslateEvent::Words(post.clone())))
}

/// `menu` with the N16–N20 items of the post.
pub(super) fn with_thought_items(menu: PopupMenu, controls: &DataControls, _window: &mut Window, _cx: &mut Context<PopupMenu>) -> PopupMenu {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    if controls.locked {
        return menu;
    }
    let mut menu = translation_items(menu.item(item(tr("posts.lens.menu"), LucideIcon::Layers, thought(ThoughtEvent::Lens(LensEvent::Open(post.clone()))), on_event)), controls); // N16, N20
    if let Some(question) = &controls.facts.answers {
        let event = thought(ThoughtEvent::Questions(QuestionsEvent::Resolve(question.clone(), Some(post.clone()))));
        menu = menu.item(item(tr("posts.open_question.menu_answers"), LucideIcon::CircleCheck, event, on_event)); // N18
    }
    if controls.own { mind_items(question_items(glossary_items(menu, controls), controls), controls) } else { menu }
}

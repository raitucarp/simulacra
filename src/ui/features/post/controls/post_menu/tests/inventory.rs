//! U3.8: every leaf action of the old post menu is reachable in the new one, on a card where
//! the old menu offered it; and what the old menu hid stays hidden.

use super::*;
use crate::domain::post_actions::ActionKind;
use crate::domain::preferences::{ArchiveAfter, Choice};
use crate::ui::features::post::controls::post_menu::post_menu_spec;
use crate::ui::menus::post_ids as ids;

/// One leaf of the old menu: its old path, its id now, and a card the old menu showed it on.
type Leaf = (&'static str, String, fn() -> PostMenuSource);

fn leaf(old: &'static str, id: &str, card: fn() -> PostMenuSource) -> Leaf {
    (old, id.to_string(), card)
}

fn fixed_leaves() -> Vec<Leaf> {
    vec![
        leaf("Reply", ids::REPLY, others), leaf("Quote", ids::QUOTE, others), leaf("Find similar", ids::FIND_SIMILAR, others),
        leaf("Copy › Copy text", ids::COPY_TEXT, others), leaf("Copy › Copy link", ids::COPY_LINK, others), leaf("Copy › Copy embed", ids::COPY_EMBED, others),
        leaf("Learn › Add question", ids::LEARN_ADD_QUESTION, others), leaf("Learn › Add answer", ids::LEARN_ADD_ANSWER, others),
        leaf("Learn › Edit review", ids::LEARN_EDIT, learning), leaf("Learn › Stop learning", ids::LEARN_STOP, learning),
        leaf("Learn › Learn this thread's connections", ids::LEARN_CONNECTIONS, others), leaf("Set a credence…", ids::SET_CREDENCE, own),
        leaf("Edit", ids::EDIT, own), leaf("Bookmark", ids::BOOKMARK, others), leaf("Remove bookmark", ids::UNBOOKMARK, || card(false, |c| c.bookmarked = true)),
        leaf("Organize › Archive", ids::ARCHIVE, own), leaf("Organize › Unarchive", ids::UNARCHIVE, archived),
        leaf("Organize › Delete automatically › Never", ids::AUTO_DELETE_NEVER, own),
        leaf("Organize › Plant in the garden", ids::GARDEN_PLANT, own), leaf("Organize › Garden › Grow to …", ids::GARDEN_GROW, planted),
        leaf("Organize › Garden › Back to …", ids::GARDEN_CUT, planted), leaf("Organize › Garden › Return to the stream", ids::GARDEN_RETURN, planted),
        leaf("Organize › Re-pin the quote to its source", ids::REPIN_QUOTE, repinnable), leaf("Organize › Add to a series › <series>", ids::SERIES_ADD, own),
        leaf("Organize › Add to a series › New series…", ids::SERIES_NEW, own), leaf("Organize › Leave “…”", ids::SERIES_LEAVE, in_series),
        leaf("Organize › Who can read this › Everyone", ids::AUDIENCE_EVERYONE, own), leaf("Organize › Who can read this › Only me", ids::AUDIENCE_ONLY_ME, own),
        leaf("Organize › Who can read this › <agent>", ids::AUDIENCE_AGENT, own), leaf("Organize › History (N)", ids::HISTORY, edited),
        leaf("Organize › Add to collection › <collection>", ids::COLLECTION_TOGGLE, others), leaf("Organize › Add to collection › New collection…", ids::COLLECTION_NEW, others),
        leaf("Think it over › Transcribe voice note", ids::TRANSCRIBE, voice), leaf("Think it over › Retract…", ids::RETRACT, own),
        leaf("Think it over › Mark as corrected…", ids::MARK_CORRECTED, own), leaf("Think it over › Take back the correction", ids::WITHDRAW_CORRECTION, corrected),
        leaf("Think it over › Distill thread to vault", ids::DISTILL_THREAD, others), leaf("Think it over › Distill to vault with a summary", ids::DISTILL_SUMMARY, others),
        leaf("Think it over › Lens fan…", ids::LENS_FAN, others), leaf("Think it over › Show in <language>", ids::TRANSLATE_SHOW, others),
        leaf("Think it over › Show original", ids::TRANSLATE_ORIGINAL, translated), leaf("Think it over › Send words to review…", ids::WORDS_TO_REVIEW, translated),
        leaf("Think it over › This answers my question", ids::QUESTION_ANSWERS, answering), leaf("Think it over › Define a term…", ids::DEFINE_TERM, own),
        leaf("Think it over › Stop defining “…”", ids::UNDEFINE_TERM, defining), leaf("Think it over › Make it an open question", ids::QUESTION_ASK, own),
        leaf("Think it over › Mark answered", ids::QUESTION_RESOLVE, || question(QuestionState::Open)), leaf("Think it over › Not a question any more", ids::QUESTION_UNASK, || question(QuestionState::Open)),
        leaf("Think it over › Reopen the question", ids::QUESTION_REOPEN, || question(QuestionState::Answered)), leaf("Think it over › Changed my mind…", ids::CHANGED_MIND, own),
        leaf("Think it over › Not a change of mind", ids::UNMARK_MIND, changed_mind), leaf("Think it over › Ask the room", ids::ASK_ROOM, own),
        leaf("Think it over › Reply in the margin…", ids::MARGIN, others), leaf("Think it over › Fork a what-if…", ids::BRANCH, others),
        leaf("Delete", ids::DELETE, own),
    ]
}

/// The families with a fixed set of members: AI actions, archive and delete timers, entry moves.
fn family_leaves() -> Vec<Leaf> {
    let actions = ActionKind::ALL.iter().map(|kind| leaf("Actions › <action>", &format!("{}{}", ids::AI_PREFIX, kind.key()), others));
    let archive = ArchiveAfter::ALL.iter().map(|after| leaf("Organize › Archive automatically › <after>", &format!("{}{}", ids::AUTO_ARCHIVE_PREFIX, after.key()), own));
    let delete = crate::domain::post::auto_delete::DeleteAfter::ALL.iter().map(|after| leaf("Organize › Delete automatically › <delay>", &format!("{}{}", ids::AUTO_DELETE_PREFIX, after.key()), own));
    let states = crate::domain::symbols::EntryState::ALL.into_iter().filter(|to| crate::domain::symbols::can_transition(crate::domain::symbols::EntrySymbol::Task, crate::domain::symbols::EntryState::Open, *to));
    let entry = states.map(|to| leaf("Organize › Entry › <state>", &format!("{}{}", ids::ENTRY_PREFIX, crate::domain::symbols::state_key(to)), task));
    actions.chain(archive).chain(delete).chain(entry).collect()
}

#[test]
fn every_action_of_the_old_menu_is_reachable_where_it_was() {
    let leaves: Vec<Leaf> = fixed_leaves().into_iter().chain(family_leaves()).collect();
    assert!(leaves.len() >= 70, "the inventory lists every old leaf ({})", leaves.len());
    let mut missing = Vec::new();
    for (old, id, card) in &leaves {
        match post_menu_spec(&card()).path_of(id) {
            Some(path) => println!("{old} => {}", path.join(" › ")),
            None => missing.push(format!("{old} ({id})")),
        }
    }
    assert!(missing.is_empty(), "unreachable: {missing:#?}");
}

#[test]
fn what_the_old_menu_hid_on_a_locked_post_stays_hidden() {
    let spec = post_menu_spec(&locked());
    let hidden = [ids::MARGIN, ids::BRANCH, ids::LENS_FAN, ids::TRANSLATE_SHOW, ids::RETRACT, ids::MARK_CORRECTED, ids::DEFINE_TERM, ids::QUESTION_ASK, ids::CHANGED_MIND, ids::ASK_ROOM];
    for id in hidden {
        assert!(!spec.reaches(id), "{id} is offered on a locked post");
    }
    assert!(!spec.reaches(&format!("{}explain", ids::AI_PREFIX)), "a locked post is never sent to a model");
    for id in [ids::REPLY, ids::EDIT, ids::BOOKMARK, ids::DISTILL_THREAD, ids::DELETE] {
        assert!(spec.reaches(id), "{id} stays on a locked post");
    }
}

#[test]
fn own_only_actions_stay_off_other_peoples_posts() {
    let spec = post_menu_spec(&others());
    for id in [ids::EDIT, ids::DELETE, ids::ARCHIVE, ids::AUTO_DELETE_NEVER, ids::AUDIENCE_EVERYONE, ids::GARDEN_PLANT, ids::SERIES_NEW, ids::RETRACT, ids::DEFINE_TERM, ids::QUESTION_ASK, ids::CHANGED_MIND, ids::ASK_ROOM, ids::SET_CREDENCE] {
        assert!(!spec.reaches(id), "{id} is offered on someone else's post");
    }
    assert!(!post_menu_spec(&card(true, |c| c.facts.asked = true)).reaches(ids::ASK_ROOM), "the room is asked once");
    assert!(!post_menu_spec(&card(true, |c| c.facts.feeling = true)).reaches(ids::QUESTION_ASK), "a feeling is never a question");
    assert!(!post_menu_spec(&archived()).reaches(&format!("{}{}", ids::AUTO_ARCHIVE_PREFIX, ArchiveAfter::ALL[0].key())), "an archived post has no timer");
}

#[test]
fn ticks_show_the_current_choice() {
    let spec = post_menu_spec(&card(true, |c| c.member_of = vec![crate::domain::document::CollectionId::new("c1")]));
    assert_eq!(spec.item(ids::COLLECTION_TOGGLE).and_then(|item| item.checked), Some(true));
    assert_eq!(spec.item(ids::AUDIENCE_EVERYONE).and_then(|item| item.checked), Some(true), "everyone reads a post by default");
    assert_eq!(spec.item(ids::AUDIENCE_ONLY_ME).and_then(|item| item.checked), Some(false));
    assert_eq!(spec.item(ids::AUTO_DELETE_NEVER).and_then(|item| item.checked), Some(true), "a post keeps itself by default");
}

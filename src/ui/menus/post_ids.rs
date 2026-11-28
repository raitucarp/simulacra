//! Stable identifiers of the post menu's actions and submenus. The inventory test proves each
//! one of the old menu is still reachable; the M1 decision maps every one to its new path.
//! A family (one item per collection, per series, per agent) shares one id; a family with a
//! fixed set of members (AI actions, timers, entry moves) suffixes the member's key.

pub const REPLY: &str = "post.reply";
pub const QUOTE: &str = "post.quote";
pub const MARGIN: &str = "post.margin";

pub const THINK: &str = "post.think";
/// `post.ai.<action key>`, under `post.ai_group.<group key>`.
pub const AI_PREFIX: &str = "post.ai.";
pub const AI_GROUP_PREFIX: &str = "post.ai_group.";
pub const FIND_SIMILAR: &str = "post.find_similar";
pub const BRANCH: &str = "post.branch";

pub const STUDY: &str = "post.study";
pub const LEARN_ADD_QUESTION: &str = "post.learn.add_question";
pub const LEARN_ADD_ANSWER: &str = "post.learn.add_answer";
pub const LEARN_EDIT: &str = "post.learn.edit";
pub const LEARN_STOP: &str = "post.learn.stop";
pub const LEARN_CONNECTIONS: &str = "post.learn.connections";
pub const WORDS_TO_REVIEW: &str = "post.translation.words";
pub const QUESTION_ASK: &str = "post.question.ask";
pub const QUESTION_RESOLVE: &str = "post.question.resolve";
pub const QUESTION_UNASK: &str = "post.question.unask";
pub const QUESTION_REOPEN: &str = "post.question.reopen";
pub const QUESTION_ANSWERS: &str = "post.question.answers";
pub const ASK_ROOM: &str = "post.question.ask_room";
pub const DEFINE_TERM: &str = "post.glossary.define";
pub const UNDEFINE_TERM: &str = "post.glossary.undefine";
pub const SET_CREDENCE: &str = "post.credence.set";

pub const ORGANIZE: &str = "post.organize";
pub const BOOKMARK: &str = "post.bookmark";
pub const UNBOOKMARK: &str = "post.unbookmark";
pub const COLLECTIONS: &str = "post.collections";
pub const COLLECTION_TOGGLE: &str = "post.collection.toggle";
pub const COLLECTION_NEW: &str = "post.collection.new";
pub const SERIES: &str = "post.series";
pub const SERIES_ADD: &str = "post.series.add";
pub const SERIES_NEW: &str = "post.series.new";
pub const SERIES_LEAVE: &str = "post.series.leave";
pub const GARDEN: &str = "post.garden";
pub const GARDEN_PLANT: &str = "post.garden.plant";
pub const GARDEN_GROW: &str = "post.garden.grow";
pub const GARDEN_CUT: &str = "post.garden.cut";
pub const GARDEN_RETURN: &str = "post.garden.return";
pub const ENTRY: &str = "post.entry";
/// `post.entry.<state key>`.
pub const ENTRY_PREFIX: &str = "post.entry.";
pub const REPIN_QUOTE: &str = "post.quote.repin";
pub const DISTILL_THREAD: &str = "post.distill.thread";
pub const DISTILL_SUMMARY: &str = "post.distill.summary";
pub const ARCHIVE: &str = "post.archive";
pub const UNARCHIVE: &str = "post.unarchive";
pub const AUTO_ARCHIVE: &str = "post.auto_archive";
/// `post.auto_archive.<choice key>`.
pub const AUTO_ARCHIVE_PREFIX: &str = "post.auto_archive.";
pub const AUTO_DELETE: &str = "post.auto_delete";
pub const AUTO_DELETE_NEVER: &str = "post.auto_delete.never";
/// `post.auto_delete.<delay key>`.
pub const AUTO_DELETE_PREFIX: &str = "post.auto_delete.";

pub const VIEW: &str = "post.view";
pub const TRANSLATE_SHOW: &str = "post.translation.show";
pub const TRANSLATE_ORIGINAL: &str = "post.translation.original";
pub const LENS_FAN: &str = "post.lens";
pub const TRANSCRIBE: &str = "post.voice.transcribe";

pub const SHARE: &str = "post.share";
pub const COPY_TEXT: &str = "post.copy.text";
pub const COPY_LINK: &str = "post.copy.link";
pub const COPY_EMBED: &str = "post.copy.embed";
pub const AUDIENCE: &str = "post.audience";
pub const AUDIENCE_EVERYONE: &str = "post.audience.everyone";
pub const AUDIENCE_ONLY_ME: &str = "post.audience.only_me";
pub const AUDIENCE_AGENT: &str = "post.audience.agent";

pub const EDIT_HISTORY: &str = "post.edit_history";
pub const EDIT: &str = "post.edit";
pub const HISTORY: &str = "post.history";
pub const MARK_CORRECTED: &str = "post.correction.corrected";
pub const WITHDRAW_CORRECTION: &str = "post.correction.withdraw";
pub const CHANGED_MIND: &str = "post.mind.open";
pub const UNMARK_MIND: &str = "post.mind.unmark";
pub const RETRACT: &str = "post.correction.retract";

pub const DELETE: &str = "post.delete";

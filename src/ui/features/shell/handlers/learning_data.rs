//! The Learn tab's data (sb E2), built when the page's data is: the network's records, each
//! goal's progress, forecast and radar, the overall radar and the weekly digest, from the
//! deck, the learner's own posts and the vault.

use super::super::view::AppView;
use crate::domain::learning::concept::{concepts_of, dedup};
use crate::domain::learning::gaps::{gap_report, GapInputs, ItemText, SourceText};
use crate::domain::learning::goals::progress::is_goal_post;
use crate::domain::learning::goals::{expected_share, forecast, goal_progress, GoalInputs, OwnPost};
use crate::domain::learning::weekly::{weekly_digest, WeekInputs};
use crate::domain::learning::{Goal, LearningRecords};
use crate::domain::review::day::review_day;
use crate::domain::review::{Deck, Policy};
use crate::domain::post::{WIKILINK_CLOSE, WIKILINK_OPEN};
use crate::domain::Post;
use crate::ui::features::review::learning::{GoalView, LearningData};
use jiff::civil::Date;

/// Characters of a vault document the radar reads (its title and opening).
const SOURCE_CHARS: usize = 4_000;
/// Replies walked up at most to find a thread's first post.
const MAX_THREAD_DEPTH: usize = 64;

/// `post`'s text without its wikilinks to other posts (links to thoughts, not concepts).
fn without_post_links(post: &Post) -> String {
    post.linked_posts.iter().fold(post.content.clone(), |text, id| text.replace(&format!("{WIKILINK_OPEN}{}{WIKILINK_CLOSE}", id.as_str()), ""))
}

/// What the learning computations read, gathered once.
struct LearningInputs {
    own_posts: Vec<OwnPost>,
    item_texts: Vec<ItemText>,
    sources: Vec<SourceText>,
}

impl AppView {
    /// The first post of `post`'s thread.
    pub(in crate::ui::features::shell) fn thread_root(&self, post: &Post) -> Post {
        let mut root = post.clone();
        for _ in 0..MAX_THREAD_DEPTH {
            match root.reply_to_id.as_ref().and_then(|id| self.state.get_post(id)) {
                Some(parent) => root = parent.clone(),
                None => break,
            }
        }
        root
    }

    /// The active network's posts written by the learner (as themselves or a persona).
    fn own_posts(&self) -> Vec<OwnPost> {
        let own = self.state.current_network_posts().filter(|post| !post.is_instant_repost() && self.state.author_of(post).is_human_persona());
        own.map(|post| OwnPost { id: post.id.as_str().to_string(), root_id: self.thread_root(post).id.as_str().to_string(), text: without_post_links(post), day: review_day(&crate::time::local(post.created_at)) }).collect()
    }

    fn learning_inputs(&self, deck: &Deck, today: Date, policy: &Policy) -> LearningInputs {
        let text_of = |id: &str| self.state.get_post_by_id(id).map(|post| post.content.clone());
        let item_texts = deck.items.iter().filter(|item| !item.retired).map(|item| ItemText {
            text: format!("{} {}", item.prompt_text(&text_of), item.answer_text(&text_of)),
            tags: item.tags.clone(),
            retrievability: item.schedule.retrievability(today, &policy.params),
            last_review: item.schedule.last_review,
        });
        let sources = self.state.current_documents().map(|doc| SourceText { title: doc.title.clone(), text: doc.content.chars().take(SOURCE_CHARS).collect() });
        LearningInputs { own_posts: self.own_posts(), item_texts: item_texts.collect(), sources: sources.collect() }
    }

    /// A goal's concepts and those of its linked posts (its radar's concepts).
    fn goal_concepts(goal: &Goal, own_posts: &[OwnPost]) -> Vec<String> {
        let linked = own_posts.iter().filter(|post| is_goal_post(goal, post)).flat_map(|post| concepts_of(&post.text));
        dedup(goal.concepts().into_iter().chain(linked))
    }

    /// The Learn tab's data for the active network.
    pub(in crate::ui::features::shell) fn learning_data(&self, deck: &Deck, today: Date, policy: &Policy) -> LearningData {
        let records: LearningRecords = self.review.learning.records.of_network(&self.state.active_network_id);
        let inputs = self.learning_inputs(deck, today, policy);
        let gap_inputs = GapInputs { own_posts: &inputs.own_posts, sources: &inputs.sources, items: &inputs.item_texts, sessions: &records.sessions, pinned: &[], only: None, today };
        let goal_inputs = GoalInputs { items: &deck.items, own_posts: &inputs.own_posts, sessions: &records.sessions, today, params: &policy.params };
        let goals = records.goals.iter().map(|goal| {
            let progress = goal_progress(goal, &goal_inputs);
            GoalView { forecast: forecast(goal, &progress, today), expected: expected_share(goal, today), goal: goal.clone(), progress }
        });
        let goal_gaps = records.goals.iter().map(|goal| {
            let concepts = Self::goal_concepts(goal, &inputs.own_posts);
            (goal.id.clone(), gap_report(&GapInputs { only: Some(&concepts), pinned: &concepts, ..gap_inputs }))
        });
        let digest = weekly_digest(&WeekInputs { items: &deck.items, log: &deck.log, own_posts: &inputs.own_posts, sessions: &records.sessions, today });
        LearningData {
            goals: goals.collect(),
            goal_gaps: goal_gaps.collect(),
            gaps: gap_report(&gap_inputs),
            digest,
            documents: self.state.current_documents().map(|doc| (doc.id.clone(), doc.title.clone())).collect(),
            tutor: self.review.learning.tutor.clone(),
            records,
        }
    }
}

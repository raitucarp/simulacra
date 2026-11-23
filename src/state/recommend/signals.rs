//! What the store says about a profile's interests (its engagement [`Signal`]s), and which
//! posts it may be recommended.

use crate::domain::reaction::Reaction;
use crate::domain::recommend::{Candidate, Signal};
use crate::domain::{NetworkId, Post, PostId, ProfileId};
use crate::state::AppState;
use std::collections::HashSet;

/// How much each kind of engagement says about an interest.
const WROTE: f32 = 1.0;
const ANSWERED: f32 = 0.7;
const QUOTED: f32 = 0.7;
const REPOSTED: f32 = 0.6;
const LIKED: f32 = 0.5;
/// How much a reaction says: an emoji counts like a like; claps start there and grow with
/// each clap up to a cap.
const REACTED: f32 = 0.5;
const CLAP_STEP: f32 = 0.05;
const CLAPPED_MAX: f32 = 0.8;

/// The weight of `reaction`'s engagement.
pub fn reaction_weight(reaction: &Reaction) -> f32 {
    match reaction.is_clap() {
        true => (REACTED + CLAP_STEP * reaction.count.saturating_sub(1) as f32).min(CLAPPED_MAX),
        false => REACTED,
    }
}

/// The reactions `profile` gave to posts of `network`, as signals at the moment of each.
fn reaction_signals(state: &AppState, profile: &ProfileId, network: &NetworkId) -> Vec<Signal> {
    let posts: HashSet<&PostId> = network_posts(state, network).map(|post| &post.id).collect();
    state.reactions.given_by(profile).filter(|row| posts.contains(&row.post_id)).map(|row| Signal { post: row.post_id.clone(), weight: reaction_weight(row), at: row.updated_at }).collect()
}

/// Posts of `network`, newest first (store order).
pub fn network_posts<'a>(state: &'a AppState, network: &'a NetworkId) -> impl Iterator<Item = &'a Post> + 'a {
    state.posts.iter().filter(move |post| &post.network_id == network)
}

/// Every engagement of `profile` in `network`: posts it wrote, answered, quoted or reposted,
/// reacted to or clapped for, and (for the profile the user is acting as) the posts the user
/// liked or reposted.
pub fn signals_of(state: &AppState, profile: &ProfileId, network: &NetworkId) -> Vec<Signal> {
    let acting = &state.active_profile_id == profile;
    let mut signals = Vec::new();
    for post in network_posts(state, network) {
        let mut push = |id: &PostId, weight: f32| signals.push(Signal { post: id.clone(), weight, at: post.created_at });
        if &post.author_id == profile {
            if !post.is_repost() {
                push(&post.id, WROTE);
            }
            post.reply_to_id.iter().for_each(|parent| push(parent, ANSWERED));
            post.quote_id.iter().for_each(|quoted| push(quoted, QUOTED));
            post.repost_id.iter().for_each(|original| push(original, REPOSTED));
        }
        if acting && post.is_liked {
            push(&post.id, LIKED);
        }
        if acting && post.is_reposted {
            push(&post.id, REPOSTED);
        }
    }
    signals.extend(reaction_signals(state, profile, network));
    signals
}

/// Posts of `network` that `profile` did not write, repost or already answer and may read (N8), newest first.
pub fn candidates_for(state: &AppState, profile: &ProfileId, network: &NetworkId) -> Vec<Candidate> {
    let answered: HashSet<&PostId> = network_posts(state, network).filter(|p| &p.author_id == profile).filter_map(|p| p.reply_to_id.as_ref()).collect();
    network_posts(state, network)
        .filter(|post| &post.author_id != profile && !post.is_repost() && !answered.contains(&post.id) && state.may_read(profile, post))
        .map(|post| Candidate { post: post.id.clone(), created_at: post.created_at })
        .collect()
}

/// The first post of `post`'s conversation (following parents; stops on a missing parent or
/// a cycle).
pub fn conversation_root(state: &AppState, post: &Post) -> PostId {
    let mut seen = HashSet::new();
    let mut current = post;
    while let Some(parent) = current.reply_to_id.as_ref().and_then(|id| state.get_post_by_id(id.as_str())) {
        if !seen.insert(current.id.clone()) {
            break;
        }
        current = parent;
    }
    current.id.clone()
}

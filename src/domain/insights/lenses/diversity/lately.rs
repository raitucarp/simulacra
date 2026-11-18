//! What was mostly read lately (the analysed posts of the last [`LATELY_DAYS`] days): the
//! busiest author and stance and their shares, the stances of the network's agents heard
//! least in that window with agents who hold them, and posts furthest in meaning from the
//! window's posts (lowest cosine to their mean direction).

use super::measures::centroid;
use super::share::{self, Dimension, ShareLabel, Shares};
use super::ProfileMap;
use crate::domain::embedding::cosine;
use crate::domain::insights::lenses::{Analysed, LensInput};
use crate::domain::trait_option::TraitOption;
use crate::domain::ProfileId;
use chrono::Duration;
use std::collections::{HashMap, HashSet};

/// Days that count as "lately".
pub const LATELY_DAYS: i64 = 14;
/// Stances, agents and posts suggested at most.
pub const SUGGESTIONS: usize = 3;

/// An agent holding a stance heard little lately.
#[derive(Debug, Clone, PartialEq)]
pub struct AgentSuggestion {
    pub id: ProfileId,
    pub name: String,
    /// Locale key of the agent's stance.
    pub stance: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lately {
    pub days: i64,
    pub posts: usize,
    /// Name and share of the busiest author.
    pub top_author: Option<(String, f64)>,
    /// Locale key and share (among agents' posts) of the busiest stance.
    pub top_stance: Option<(String, f64)>,
    /// Locale keys of the least heard stances among the network's agents.
    pub quieter: Vec<String>,
    pub agents: Vec<AgentSuggestion>,
    /// Indices of analysed posts from before the window, furthest in meaning first.
    pub further: Vec<usize>,
}

fn top(shares: &Shares) -> Option<(String, f64)> {
    shares.shares.first().map(|s| (match &s.label { ShareLabel::Name(text) | ShareLabel::Key(text) => text.clone() }, shares.share(0)))
}

/// The window's summary, or `None` when nothing was posted in it.
pub fn of(input: &LensInput, analysed: &Analysed, profiles: &ProfileMap, vectors: &HashMap<&str, &[f32]>) -> Option<Lately> {
    let since = input.now - Duration::days(LATELY_DAYS);
    let (recent, earlier): (Vec<usize>, Vec<usize>) = (0..analysed.posts.len()).partition(|&i| analysed.posts[i].created_at >= since);
    if recent.is_empty() {
        return None;
    }
    let authors = Shares::of(Dimension::Author, recent.iter().map(|&i| analysed.author_of[i]).map(|a| (analysed.authors[a].id.as_str().to_string(), ShareLabel::Name(analysed.authors[a].name.clone()))));
    let stances = Shares::of(Dimension::Stance, recent.iter().filter_map(|&i| share::stance(analysed.posts[i], profiles)));
    let (quieter, agents) = quieter_stances(input, &stances);
    let further = further_posts(analysed, &recent, &earlier, vectors);
    Some(Lately { days: LATELY_DAYS, posts: recent.len(), top_author: top(&authors), top_stance: top(&stances), quieter, agents, further })
}

/// Stances of the network's agents heard less lately than the most heard one, least heard
/// first, and one agent holding each.
fn quieter_stances(input: &LensInput, heard: &Shares) -> (Vec<String>, Vec<AgentSuggestion>) {
    let agents: Vec<_> = input.profiles.iter().filter(|p| p.is_agent() && p.belongs_to_network(&input.network)).collect();
    let count = |key: &str| heard.shares.iter().find(|s| s.key == key).map_or(0, |s| s.count);
    let mut stances: Vec<_> = agents.iter().map(|a| a.config.epistemic_stance).collect::<HashSet<_>>().into_iter().collect();
    stances.sort_by_key(|s| (count(s.key()), s.key()));
    let most = heard.shares.first().map_or(0, |s| s.count);
    let quieter: Vec<_> = stances.into_iter().filter(|s| most == 0 || count(s.key()) < most).take(SUGGESTIONS).collect();
    let suggestions = quieter.iter().filter_map(|s| agents.iter().find(|a| a.config.epistemic_stance == *s)).map(|a| AgentSuggestion { id: a.id.clone(), name: a.name.clone(), stance: a.config.epistemic_stance.label_key() });
    (quieter.iter().map(|s| s.label_key()).collect(), suggestions.collect())
}

/// Earlier posts whose vectors point furthest from the recent posts' mean direction.
fn further_posts(analysed: &Analysed, recent: &[usize], earlier: &[usize], vectors: &HashMap<&str, &[f32]>) -> Vec<usize> {
    let vector = |i: usize| vectors.get(analysed.posts[i].id.as_str()).copied();
    let Some(centre) = centroid(&recent.iter().filter_map(|&i| vector(i)).collect::<Vec<_>>()) else { return Vec::new() };
    let mut scored: Vec<(usize, f32)> = earlier.iter().filter_map(|&i| vector(i).map(|v| (i, cosine(&centre, v)))).collect();
    scored.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    scored.into_iter().take(SUGGESTIONS).map(|(i, _)| i).collect()
}

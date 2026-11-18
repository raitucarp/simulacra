//! Viewpoint diversity: how varied the analysed posts are by author, by the agents' epistemic
//! stances and stylistic schools (from their configs), and in meaning (embedding spread); how
//! replies stand to what they answer; the trend per period; what was mostly read lately; and
//! how varied the vault reading is by subject and creator. It describes; it never scores
//! the reader.

pub mod lately;
pub mod measures;
pub mod share;
pub mod stance;

pub use lately::Lately;
pub use measures::Variety;
pub use share::{Dimension, Share, ShareLabel, Shares};
pub use stance::{ReplyShares, ReplyStance};

use super::periods::Periods;
use super::{Analysed, LensInput};
use crate::domain::document::record::ReadingStatus;
use crate::domain::{Post, Profile};
use measures::{mean_pairwise_distance, variety};
use std::collections::HashMap;

/// Periods of the trend at most.
pub const TREND_PERIODS: usize = 12;

/// The variety of one period's posts (Simpson index; `None` with fewer than two).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VarietyPoint {
    pub period: usize,
    pub posts: usize,
    pub authors: Option<f64>,
    pub stances: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DiversityReport {
    /// Authors, stances and schools of the analysed posts.
    pub feed: Vec<Shares>,
    /// Mean cosine distance between the analysed posts that have a vector.
    pub spread: Option<f64>,
    pub embedded: usize,
    pub replies: ReplyShares,
    pub periods: Periods,
    pub trend: Vec<VarietyPoint>,
    pub lately: Option<Lately>,
    /// Subjects and creators of the vault documents being read or read.
    pub reading: Vec<Shares>,
    pub documents_read: usize,
}

/// The analysed posts' profiles, by author id.
pub(super) type ProfileMap<'a> = HashMap<&'a str, &'a Profile>;

impl DiversityReport {
    pub fn of(input: &LensInput, analysed: &Analysed) -> Self {
        let profiles: ProfileMap = input.profiles.iter().map(|p| (p.id.as_str(), p)).collect();
        let posts = &analysed.posts;
        let vectors: HashMap<&str, &[f32]> = input.vectors.iter().map(|(id, v)| (id.as_str(), v.as_slice())).collect();
        let embedded: Vec<&[f32]> = posts.iter().filter_map(|p| vectors.get(p.id.as_str()).copied()).collect();
        let periods = posts.iter().map(|p| analysed.day(p.created_at)).min().map(|first| Periods::spanning(first, analysed.today, TREND_PERIODS)).unwrap_or_default();
        let (documents_read, reading) = reading(input);
        Self {
            feed: vec![share::authors(analysed), Shares::of(Dimension::Stance, posts.iter().filter_map(|p| share::stance(p, &profiles))), Shares::of(Dimension::School, posts.iter().filter_map(|p| share::school(p, &profiles)))],
            spread: mean_pairwise_distance(&embedded),
            embedded: embedded.len(),
            replies: replies(&input.posts, posts),
            trend: trend(analysed, &profiles, &periods),
            lately: lately::of(input, analysed, &profiles, &vectors),
            periods,
            reading,
            documents_read,
        }
    }

    /// The shares of `dimension` among the analysed posts.
    pub fn feed_shares(&self, dimension: Dimension) -> Option<&Shares> {
        self.feed.iter().find(|s| s.dimension == dimension)
    }
}

/// The stance of each analysed reply whose parent is known (in any network).
fn replies(all: &[Post], analysed: &[&Post]) -> ReplyShares {
    let by_id: HashMap<&str, &Post> = all.iter().map(|p| (p.id.as_str(), p)).collect();
    let parent = |post: &Post| post.reply_to_id.as_ref().and_then(|id| by_id.get(id.as_str()).copied());
    ReplyShares::of(analysed.iter().filter_map(|reply| parent(reply).map(|parent| stance::reply_stance(&parent.content, &reply.content))))
}

/// Simpson index of authors and of stances per period.
fn trend(analysed: &Analysed, profiles: &ProfileMap, periods: &Periods) -> Vec<VarietyPoint> {
    (0..periods.count).map(|period| {
        let within: Vec<usize> = (0..analysed.posts.len()).filter(|&i| periods.index_of(analysed.day(analysed.posts[i].created_at)) == Some(period)).collect();
        let simpson = |keys: Vec<String>| {
            let shares = Shares::counted(keys);
            (shares.iter().sum::<usize>() >= 2).then(|| variety(&shares).simpson)
        };
        let authors = simpson(within.iter().map(|&i| analysed.author_of[i].to_string()).collect());
        let stances = simpson(within.iter().filter_map(|&i| share::stance(analysed.posts[i], profiles)).map(|(key, _)| key).collect());
        VarietyPoint { period, posts: within.len(), authors, stances }
    }).collect()
}

/// The vault documents of the network being read or read, by subject and by creator.
fn reading(input: &LensInput) -> (usize, Vec<Shares>) {
    let read: Vec<_> = input.documents.iter().filter(|d| d.network_id == input.network && d.record.reading.status != ReadingStatus::Unread).collect();
    let subject_label = |id: &str| input.subjects.iter().find(|(s, _)| s.as_str() == id).map_or_else(|| id.to_string(), |(_, label)| label.clone());
    let subjects = read.iter().flat_map(|d| d.record.subjects.iter()).map(|s| (s.as_str().to_string(), ShareLabel::Name(subject_label(s.as_str()))));
    let creators = read.iter().flat_map(|d| d.record.creators.iter()).map(|c| (c.name.to_lowercase(), ShareLabel::Name(c.name.clone())));
    (read.len(), vec![Shares::of(Dimension::Subject, subjects), Shares::of(Dimension::Creator, creators)])
}

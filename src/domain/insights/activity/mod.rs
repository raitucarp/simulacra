//! Chart data of the Activity view: posts over time stacked by kind, per author and per kind,
//! the weekly rhythm (weekday × hour), distributions and the agents' trait profiles.

pub mod histogram;
pub mod timeline;
#[cfg(test)]
mod tests;

pub use histogram::{histogram, Histogram};
pub use timeline::{timeline, TimeBucket};

use super::facts::{AuthorFacts, PostFacts, PostKind, TRAIT_AXES};
use chrono::NaiveDate;

pub const WEEKDAYS: usize = 7;
pub const HOURS: usize = 24;
/// Authors listed in the per-author chart.
pub const TOP_AUTHORS: usize = 12;

/// Counts per kind, indexed like [`PostKind::ALL`].
pub type KindCounts = [usize; PostKind::COUNT];

/// Everything the Activity view draws.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityReport {
    pub timeline: Vec<TimeBucket>,
    /// `(author index, counts by kind)`, busiest first.
    pub authors: Vec<(usize, KindCounts)>,
    pub kinds: KindCounts,
    /// Posts per weekday (Monday first) and hour.
    pub rhythm: [[usize; HOURS]; WEEKDAYS],
    pub lengths: Histogram,
    pub sentence_lengths: Histogram,
    /// `(author index, trait values)` of the agents among the authors.
    pub traits: Vec<(usize, [u8; TRAIT_AXES])>,
}

impl ActivityReport {
    /// Builds every series from `posts` up to `today` (`start` bounds the timeline when set).
    pub fn of(posts: &[PostFacts], authors: &[AuthorFacts], start: Option<NaiveDate>, today: NaiveDate) -> Self {
        let mut kinds = [0usize; PostKind::COUNT];
        let mut rhythm = [[0usize; HOURS]; WEEKDAYS];
        let mut per_author = vec![[0usize; PostKind::COUNT]; authors.len()];
        for post in posts {
            kinds[post.kind.index()] += 1;
            rhythm[post.weekday.min(WEEKDAYS - 1)][post.hour.min(HOURS - 1)] += 1;
            if let Some(counts) = per_author.get_mut(post.author) {
                counts[post.kind.index()] += 1;
            }
        }
        let mut ranked: Vec<(usize, KindCounts)> = per_author.into_iter().enumerate().filter(|(_, c)| c.iter().sum::<usize>() > 0).collect();
        ranked.sort_by_key(|(author, counts)| (std::cmp::Reverse(counts.iter().sum::<usize>()), *author));
        ranked.truncate(TOP_AUTHORS);
        let lengths: Vec<f64> = posts.iter().filter(|p| p.chars > 0).map(|p| p.chars as f64).collect();
        let sentences: Vec<f64> = posts.iter().filter(|p| p.words > 0).map(|p| p.words_per_sentence).collect();
        let traits = authors.iter().enumerate().filter_map(|(i, a)| a.traits.map(|t| (i, t))).collect();
        Self {
            timeline: timeline(posts, start, today),
            authors: ranked,
            kinds,
            rhythm,
            lengths: histogram(&lengths),
            sentence_lengths: histogram(&sentences),
            traits,
        }
    }

    /// The busiest weekday and hour, if any post was made.
    pub fn peak(&self) -> Option<(usize, usize, usize)> {
        let cells = (0..WEEKDAYS).flat_map(|d| (0..HOURS).map(move |h| (d, h)));
        cells.map(|(d, h)| (d, h, self.rhythm[d][h])).filter(|c| c.2 > 0).max_by_key(|&(d, h, c)| (c, std::cmp::Reverse((d, h))))
    }
}

//! The Readability view's data: every analysed post long enough to score (reposts carry no
//! text of their own), the median on the shared ease scale, the bands per author, the median
//! per period, and the hardest and easiest posts.

use super::periods::Periods;
use super::Analysed;
use crate::domain::readability::{readability, Band, Formula, Readability, ReadingLanguage};
use jiff::civil::Date;

/// Authors shown in the per-author distribution (the most prolific).
pub const TOP_AUTHORS: usize = 12;
/// Posts listed as hardest and as easiest.
pub const EXTREMES: usize = 6;
/// Periods of the trend at most.
pub const TREND_PERIODS: usize = 24;

/// One scored post.
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredPost {
    /// Index among the analysed posts.
    pub post: usize,
    pub author: usize,
    pub day: Date,
    pub score: Readability,
}

/// An author's posts per band and their median ease.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorReadability {
    pub author: usize,
    pub bands: [usize; Band::ALL.len()],
    pub median_ease: f64,
}

/// The median ease of one period's posts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EasePoint {
    pub period: usize,
    pub median_ease: Option<f64>,
    pub posts: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReadabilityReport {
    pub scored: Vec<ScoredPost>,
    pub median_ease: Option<f64>,
    /// Median Flesch–Kincaid grade of the English-scored posts.
    pub median_grade: Option<f64>,
    /// Share of scored posts in a hard band.
    pub hard_share: f64,
    /// Formulas used, most used first.
    pub formulas: Vec<(Formula, usize)>,
    pub authors: Vec<AuthorReadability>,
    pub periods: Periods,
    pub trend: Vec<EasePoint>,
    /// Indices into `scored`: lowest ease first, and highest ease first.
    pub hardest: Vec<usize>,
    pub easiest: Vec<usize>,
}

/// The median of `values` (`None` when empty).
pub fn median(mut values: Vec<f64>) -> Option<f64> {
    values.sort_by(f64::total_cmp);
    let n = values.len();
    match n {
        0 => None,
        _ if n % 2 == 1 => Some(values[n / 2]),
        _ => Some((values[n / 2 - 1] + values[n / 2]) / 2.0),
    }
}

impl ReadabilityReport {
    pub fn of(analysed: &Analysed, language: ReadingLanguage) -> Self {
        let scored: Vec<ScoredPost> = analysed.posts.iter().enumerate().filter(|(_, p)| !p.is_repost()).filter_map(|(i, p)| {
            readability(&p.content, language).map(|score| ScoredPost { post: i, author: analysed.author_of[i], day: analysed.day(p.created_at), score })
        }).collect();
        let mut order: Vec<usize> = (0..scored.len()).collect();
        order.sort_by(|&a, &b| scored[a].score.ease.total_cmp(&scored[b].score.ease).then(a.cmp(&b)));
        let periods = scored.iter().map(|s| s.day).min().map(|first| Periods::spanning(first, analysed.today, TREND_PERIODS)).unwrap_or_default();
        let trend = (0..periods.count).map(|period| {
            let eases: Vec<f64> = scored.iter().filter(|s| periods.index_of(s.day) == Some(period)).map(|s| s.score.ease).collect();
            EasePoint { period, posts: eases.len(), median_ease: median(eases) }
        }).collect();
        Self {
            median_ease: median(scored.iter().map(|s| s.score.ease).collect()),
            median_grade: median(scored.iter().filter_map(|s| s.score.grade).collect()),
            hard_share: if scored.is_empty() { 0.0 } else { scored.iter().filter(|s| s.score.band.is_hard()).count() as f64 / scored.len() as f64 },
            formulas: formulas_used(&scored),
            authors: by_author(&scored),
            hardest: order.iter().take(EXTREMES).copied().collect(),
            easiest: order.iter().rev().take(EXTREMES).copied().collect(),
            periods,
            trend,
            scored,
        }
    }
}

fn formulas_used(scored: &[ScoredPost]) -> Vec<(Formula, usize)> {
    let mut used: Vec<(Formula, usize)> = Formula::ALL.iter().map(|f| (*f, scored.iter().filter(|s| s.score.formula == *f).count())).filter(|(_, n)| *n > 0).collect();
    used.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    used
}

/// Bands per author, the most prolific first (ties by author index).
fn by_author(scored: &[ScoredPost]) -> Vec<AuthorReadability> {
    let mut authors: Vec<usize> = scored.iter().map(|s| s.author).collect();
    authors.sort_unstable();
    authors.dedup();
    let mut rows: Vec<AuthorReadability> = authors.into_iter().map(|author| {
        let own: Vec<&ScoredPost> = scored.iter().filter(|s| s.author == author).collect();
        let mut bands = [0usize; Band::ALL.len()];
        own.iter().for_each(|s| bands[s.score.band.index()] += 1);
        AuthorReadability { author, bands, median_ease: median(own.iter().map(|s| s.score.ease).collect()).unwrap_or_default() }
    }).collect();
    rows.sort_by(|a, b| b.bands.iter().sum::<usize>().cmp(&a.bands.iter().sum::<usize>()).then(a.author.cmp(&b.author)));
    rows.truncate(TOP_AUTHORS);
    rows
}

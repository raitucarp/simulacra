//! Post search through the vault (Q3): ranked full-text matches.

use super::Vault;
use crate::domain::PostId;
use crate::error::Result;

impl Vault {
    /// At most `limit` posts matching the FTS5 `expression`, best first, with their bm25
    /// score (lower is better), after every queued write.
    pub fn score_posts(&self, expression: &str, limit: usize) -> Result<Vec<(PostId, f64)>> {
        let expression = expression.to_string();
        self.read(move |db| db.score_posts(&expression, limit))
    }
}

impl Vault {
    /// At most `limit` earlier post versions matching the FTS5 `expression`, best first: the
    /// post and the version's text (Q6; sealed versions are never indexed).
    pub fn score_revisions(&self, expression: &str, limit: usize) -> Result<Vec<(PostId, String, f64)>> {
        let expression = expression.to_string();
        self.read(move |db| db.score_revisions(&expression, limit))
    }
}

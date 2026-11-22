//! Running code blocks in the store (epic P): the cached runtime detection (P1) and the runs
//! saved on posts (P6).

use crate::domain::code::RuntimeCache;
use crate::state::AppState;
use crate::storage::SettingKey;

impl AppState {
    /// What the last runtime detection found (never detected without a vault).
    pub fn runtime_cache(&self) -> RuntimeCache {
        let raw = self.db.as_ref().and_then(|db| db.get_setting(SettingKey::CodeRuntimes).ok().flatten());
        raw.map(|raw| RuntimeCache::from_json(&raw)).unwrap_or_default()
    }

    /// Stores a detection's result.
    pub fn store_runtime_cache(&self, cache: &RuntimeCache) {
        if let Some(db) = self.db.as_ref() {
            let _ = db.set_setting(SettingKey::CodeRuntimes, &cache.to_json());
        }
    }
}

impl AppState {
    /// Keeps `run` as the latest run of its block on post `id` (P6), saved with the post as it
    /// is: never a new version. Refused for a locked post (its output could echo the sealed text).
    pub fn save_code_run(&mut self, id: &crate::domain::PostId, run: crate::domain::code::CodeRun) -> crate::error::Result<()> {
        let position = self.index.position(id.as_str()).filter(|at| self.posts.get(*at).is_some_and(|post| &post.id == id));
        let position = position.ok_or_else(|| crate::error::AppError::DatabaseError(format!("post '{id}' not found")))?;
        let post = &mut self.posts[position];
        if post.is_locked() {
            return Err(crate::error::AppError::PostLocked);
        }
        post.extras.code_runs = crate::domain::code::with_run(std::mem::take(&mut post.extras.code_runs), run);
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

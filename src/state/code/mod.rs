//! Running code blocks in the store (epic P): the cached runtime detection (P1).

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

#[cfg(test)]
mod tests {
    use crate::domain::code::{DetectedRuntime, Runtime, RuntimeCache};
    use crate::state::AppState;

    #[test]
    fn the_detection_is_cached_in_the_vault() {
        let state = AppState::new_in_memory();
        assert_eq!(state.runtime_cache(), RuntimeCache::default(), "never detected");
        let cache = RuntimeCache { found: vec![DetectedRuntime { runtime: Runtime::Node, path: "node".into(), version: "20.1.0".into() }], probed_at: Some(jiff::Timestamp::now()) };
        state.store_runtime_cache(&cache);
        assert_eq!(state.runtime_cache(), cache);
    }
}

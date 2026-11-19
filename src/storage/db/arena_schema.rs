//! The tables of the Arena and of agent belief memory (sb F1), created idempotently on every
//! open (no data is rewritten, so no `user_version` step):
//! - `arena_debates`, `arena_experiments`: one saved debate or experiment per row, as JSON;
//! - `agent_beliefs`: one belief per row, by agent;
//! - `belief_revisions`: the append-only revision log, by agent and time;
//! - `belief_settings`: each agent's update switch and today's update count.

/// Table and index definitions.
pub(super) const CREATE_ARENA: &str = r#"
                CREATE TABLE IF NOT EXISTS arena_debates (
                    id TEXT PRIMARY KEY,
                    updated_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS arena_experiments (
                    id TEXT PRIMARY KEY,
                    updated_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS agent_beliefs (
                    id TEXT PRIMARY KEY,
                    agent_id TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_agent_beliefs_agent ON agent_beliefs(agent_id);

                CREATE TABLE IF NOT EXISTS belief_revisions (
                    seq INTEGER PRIMARY KEY AUTOINCREMENT,
                    belief_id TEXT NOT NULL,
                    agent_id TEXT NOT NULL,
                    at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_belief_revisions_agent ON belief_revisions(agent_id, at);

                CREATE TABLE IF NOT EXISTS belief_settings (
                    agent_id TEXT PRIMARY KEY,
                    settings_json TEXT NOT NULL
                );
                "#;

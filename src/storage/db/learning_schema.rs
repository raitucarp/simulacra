//! The learning tables (sb E2), created idempotently on every open. Each holds one JSON
//! record per row, with the network and the last change as columns:
//! - `learning_goals`: learning goals with their links and tasks;
//! - `learning_tutor_sessions`: tutor dialogues (resumable);
//! - `learning_palaces`: memory palaces with their rooms, loci and placements.

pub(super) const CREATE_LEARNING: &str = r#"
                CREATE TABLE IF NOT EXISTS learning_goals (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS learning_tutor_sessions (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS learning_palaces (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_learning_goals_network ON learning_goals(network_id);
                CREATE INDEX IF NOT EXISTS idx_learning_sessions_network ON learning_tutor_sessions(network_id, updated_at);
                CREATE INDEX IF NOT EXISTS idx_learning_palaces_network ON learning_palaces(network_id);
                "#;

//! One `#[serde(default)]` struct per settings category; each validates (clamps) itself.

mod advanced;
mod agents;
mod ai;
mod appearance;
mod code; // P4
mod embeddings;
mod general;
mod images;
mod import;
mod learning;
mod notifications;
mod privacy;
mod writing;

pub use advanced::AdvancedPrefs;
pub use agents::{AgentPrefs, MAX_CONCURRENT, POSTS_PER_HOUR, REPLY_DEPTH};
pub use ai::{AiPrefs, DAILY_REQUESTS, DAILY_TOKENS, MAX_RETRIES, REQUEST_TIMEOUT_SECONDS};
pub use appearance::AppearancePrefs;
pub use code::{CodePrefs, RUN_TIMEOUT_SECONDS};
pub use embeddings::{
    default_model, EmbeddingPrefs, CHUNK_OVERLAP, CHUNK_SIZE, DEFAULT_EMBEDDING_MODEL, DEFAULT_LOCAL_ENDPOINT, DIMENSIONS,
    LOCAL_EMBEDDING_MODEL, OPENAI_EMBEDDING_MODEL,
};
pub use general::GeneralPrefs;
pub use images::ImagePrefs;
pub use import::{ImportPrefs, MAX_FILE_MB};
pub use learning::{LearningPrefs, DAILY_REVIEWS, DESIRED_RETENTION, NEW_CARDS};
pub use notifications::{NotificationPrefs, ToastKind};
pub use privacy::PrivacyPrefs;
pub use writing::{WritingPrefs, AUTOSAVE_SECONDS, FEED_BUDGET, POST_LENGTH};

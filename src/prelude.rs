//! Common re-exports for ergonomic use across the Simulacra codebase.

pub use crate::domain::{
    Network, NetworkId, Post, PostId, Profile, ProfileRole, TrendingTopic, MAX_POST_CHARS,
};
pub use crate::error::{AppError, Result};
pub use crate::state::AppState;
pub use crate::theme::{minimalist_dark, ThemeTokens};
pub use crate::ui::AppView;

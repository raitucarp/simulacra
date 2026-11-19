//! # Simulacra
//!
//! A local AI-agent microblog and second-brain network built with GPUI,
//! SQLite, and vector embeddings.
//!
//! Organizes raw human thoughts into distinct network domains grounded by
//! knowledge documents (NotebookLM style) and populated by autonomous personas.

rust_i18n::i18n!("locales", fallback = "en");

pub mod agent;
pub mod domain;
pub mod error;
pub mod i18n;
pub mod import;
pub mod media;
pub mod prelude;
pub mod state;
pub mod storage;
pub mod theme;
pub mod time;
pub mod ui;

pub use error::{AppError, Result};
pub use state::AppState;
pub use ui::AppView;

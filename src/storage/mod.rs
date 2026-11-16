//! Storage and local database persistence module.

pub mod avatars;
pub mod db;
pub mod vault;

pub use db::{Database, SettingKey};
pub use vault::Vault;

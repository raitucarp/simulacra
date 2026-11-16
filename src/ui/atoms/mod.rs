//! Atomic UI primitives (the fundamental building blocks).

pub mod avatar;
pub mod avatar_cache;
pub mod badge;
pub mod char_meter;
pub mod pill;

pub use avatar::{render_avatar, render_profile_avatar};
pub use avatar_cache::forget_avatar;
pub use badge::render_role_badge;
pub use char_meter::render_char_meter;
pub use pill::{render_hashtag_span, render_obsidian_pill};

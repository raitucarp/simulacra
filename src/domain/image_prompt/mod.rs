//! Image ideas (sb D3), pure side: a guided image prompt for when the user has no picture
//! in mind, for an avatar (agent or persona) or a post.
//!
//! - [`options`]: style, mood, palette, framing and things to avoid, each with chip art.
//! - [`choices`]: the target (avatar or post) and the user's [`ImageChoices`].
//! - [`subject`]: what is drawn, from an agent's settings or a post's text.
//! - [`surprise`]: "Surprise me", a coherent random set of picks from a seed.
//! - [`engine`] / [`assemble`]: the per-engine prompt ([`assemble_prompt`]).
//! - [`candidates`]: "Write it for me", the language-model request and its parser.
//! - [`autonomous`]: the default prompt of an agent's post that came without one.
//! - [`colour_names`]: plain names of the theme colours for "match my theme".
//!
//! The effectful side (requests, files) is in the shell; the language-model runner is
//! `agent::post_actions`.

pub mod assemble;
pub mod autonomous;
pub mod candidates;
pub mod choices;
pub mod colour_names;
pub mod engine;
pub mod options;
pub mod subject;
pub mod surprise;

#[cfg(test)]
mod tests;

pub use assemble::{assemble_prompt, ImageContext, ImagePrompt, MAX_PROMPT_CHARS};
pub use autonomous::default_post_prompt;
pub use candidates::{candidates_prompt, parse_candidates, CANDIDATE_ANSWER_TOKENS, CANDIDATE_COUNT};
pub use choices::{ImageChoices, ImageTarget};
pub use colour_names::{colour_name, distinct_names};
pub use engine::{engine_for, global_engine_config, is_configured, size_of_aspect, PromptEngine};
pub use options::{ImageAvoid, ImageFraming, ImageMood, ImagePalette, ImageStyle};
pub use subject::{post_gist, subject_from_config, subject_from_post, with_article, ImageSubject};
pub use surprise::{avoid_for, moods_for, palettes_for, surprise};

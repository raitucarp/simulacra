//! Image ideas sheet (sb D3): a guided image prompt for an avatar or a post when the user has
//! no picture in mind. Art chips for style, mood, palette, framing and what to keep out,
//! "Surprise me", "Write it for me" (three ideas from the language model), the editable
//! prompt, and generation with its results. The shell owns the state and runs the requests;
//! this module only renders and reports [`IdeasEvent`]s. The prompt logic is pure
//! (`domain::image_prompt`).

mod chip;
mod failure;
mod labels;
mod model;
mod picks;
mod prompt_pane;
mod results;
mod sheet;

#[cfg(test)]
mod tests;

pub use failure::{from_action, from_image_error};
pub use labels::{failure_text, tr};
pub use model::{CandidatesStatus, GenerationStatus, IdeasDestination, IdeasEvent, IdeasFailure, IdeasPick, IdeasSheet};
pub use sheet::{render_ideas_sheet, IdeasAction, IdeasInputs};

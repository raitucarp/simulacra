//! Post symbols in the interface (epic S): the face that replaces a post's avatar (S4) and
//! the translated names of every core symbol.

mod face;
mod labels;
#[cfg(test)]
mod tests;

pub use face::{render_glyph, render_symbol_art, render_symbol_face};
pub use labels::{art_label, entry_label, glyph_label, set_symbol_meanings, signifier_label, state_label};

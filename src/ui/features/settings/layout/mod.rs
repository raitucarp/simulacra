//! The settings window layout: frame, category rail, header, body and footer.

mod body;
mod embedded;
mod footer;
mod frame;
mod header;
mod rail;

pub(super) use body::{render_category, render_search};
pub(super) use embedded::EmbeddedPanes;
pub(super) use footer::render_footer;
#[cfg(test)]
pub(super) use frame::dialog_size;
pub(super) use frame::render_frame;
pub(super) use header::{render_header, HeaderSubject};
pub(super) use rail::{render_rail, RailView};

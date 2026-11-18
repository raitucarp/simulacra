//! The Insights page (sb D2): interactive charts over the network's posts, documents,
//! concepts and people, computed by the pure analytics in `domain::insights`.
//!
//! - [`InsightsPanel`]: the page entity; the shell hands it reports (computed on the
//!   background executor, cached by a data fingerprint) and handles its [`InsightsEvent`]s.
//! - [`InsightsView`]: Overview, Network, Sets, Symmetry, Categories, Embedding map, Activity.
//! - [`PageState`]: the remembered view, filter and display choices.
//! - `chart`: the token-driven chart kit (scales, ticks, palettes, culling, hit testing,
//!   cached geometry, columns, ranked bars, legends, tooltips, summaries, canvas painting).

pub(crate) mod chart;
pub mod events;
pub mod lens_state; // sb F3: the lens views' report and interaction state
mod panel;
mod render;
pub mod readability_style; // sb F3: readability bands and formula names
pub mod state;
pub mod view;

pub use events::InsightsEvent;
pub use panel::InsightsPanel;
pub use state::PageState;
pub use view::InsightsView;

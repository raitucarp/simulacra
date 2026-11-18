//! The chart kit of the Insights page (sb D2), token-driven throughout.
//!
//! Pure geometry (unit-tested): [`scale`] (linear, band, radial), [`ticks`], [`palette`]
//! (series and ramps from theme tokens), [`cull`] (viewport culling and label placement),
//! [`spatial`] (grid hit testing), [`cache`] (geometry cached per data and size),
//! [`projection`] and [`pan_zoom`]. Elements: [`columns`] (stacked columns), [`ranked`]
//! (ranked bars), [`legend`] (toggling legends), [`tooltip`], [`summary`] (the accessible
//! line under each chart); [`paint`] holds the canvas primitives.

pub mod cache;
pub mod columns;
pub mod lanes; // sb F3: lane chart geometry (concept timeline)
pub mod cull;
pub mod legend;
pub mod paint;
pub mod palette;
pub mod pan_zoom;
pub mod projection;
pub mod ranked;
pub mod scale;
pub mod spatial;
pub mod summary;
pub mod ticks;
pub mod tooltip;

#[cfg(test)]
mod tests;

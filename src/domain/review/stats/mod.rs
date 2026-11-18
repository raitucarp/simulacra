//! Review statistics, all pure functions of the cards and the log:
//! - [`due_forecast`]: cards coming due on each of the next days;
//! - [`retention_by_week`]: the share of learnt cards recalled, week by week;
//! - [`state_counts`]: new, learning, young and mature cards;
//! - [`review_heatmap`]: answers per day over the last weeks;
//! - [`calibration`]: how often a quiz answer was right at each confidence.

mod calibration;
mod forecast;
mod heatmap;
mod retention;
mod states;

pub use calibration::{calibration, CalibrationRow};
pub use forecast::{due_forecast, FORECAST_DAYS};
pub use heatmap::{current_streak, heatmap_level, review_heatmap, HeatmapCell, HEATMAP_LEVELS};
pub use retention::{retention_by_week, RetentionPoint, RETENTION_WEEKS};
pub use states::{state_counts, CardState, StateCounts, MATURE_INTERVAL};

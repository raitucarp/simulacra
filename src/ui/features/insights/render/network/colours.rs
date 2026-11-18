//! Node colours of the Network view and the legend slots they toggle: the largest
//! communities get series colours (the rest share a muted "other"), node kinds use the Mind
//! Graph's colours, and the measure ramps from the border tone to the brand accent.

use crate::domain::graph::NODE_KINDS;
use crate::domain::insights::InsightsReport;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::palette::{mix, node_kind_colour, series, SERIES_COUNT};
use crate::ui::features::insights::state::{NetworkColour, PageState};
use gpui_kit::Rgba;


/// Legend slot of node `i` under `colouring` (community, kind or a single measure slot).
pub fn legend_slot(report: &InsightsReport, colouring: NetworkColour, i: usize) -> usize {
    match colouring {
        NetworkColour::Community => report.network.metrics.communities.of_node.get(i).copied().unwrap_or_default().min(SERIES_COUNT),
        NetworkColour::Kind => NODE_KINDS.iter().position(|k| *k == report.network.kind(i)).unwrap_or_default(),
        NetworkColour::Measure => 0,
    }
}

/// Colour of node `i`.
pub fn node_colour(report: &InsightsReport, state: &PageState, i: usize, theme: &ThemeTokens) -> Rgba {
    match state.network_colour {
        NetworkColour::Community => {
            let slot = legend_slot(report, NetworkColour::Community, i);
            if slot < SERIES_COUNT { series(slot, theme) } else { theme.text_muted }
        }
        NetworkColour::Kind => node_kind_colour(report.network.kind(i), theme),
        NetworkColour::Measure => {
            let metrics = &report.network.metrics;
            let share = (metrics.value(state.measure, i) / metrics.max(state.measure)).sqrt() as f32;
            mix(theme.border_strong, theme.accent_brand, share)
        }
    }
}

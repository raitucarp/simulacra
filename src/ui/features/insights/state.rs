//! What the Insights page remembers (one JSON row in the settings table): the open view, the
//! filter, the network scope and measure, how the network and the embedding map are
//! coloured, and the chosen set family and sets.

use super::view::InsightsView;
use crate::domain::insights::graph::metrics::Measure;
use crate::domain::insights::sets::SetDimension;
use crate::domain::insights::{GraphScope, InsightsFilter};
use serde::{Deserialize, Serialize};

/// How network nodes are coloured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkColour {
    #[default]
    Community,
    Kind,
    Measure,
}

impl NetworkColour {
    pub const ALL: [Self; 3] = [Self::Community, Self::Kind, Self::Measure];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Community => "community",
            Self::Kind => "kind",
            Self::Measure => "measure",
        }
    }
}

/// How embedding points are coloured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapColour {
    #[default]
    Cluster,
    Author,
    Kind,
    Time,
}

impl MapColour {
    pub const ALL: [Self; 4] = [Self::Cluster, Self::Author, Self::Kind, Self::Time];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Cluster => "cluster",
            Self::Author => "author",
            Self::Kind => "kind",
            Self::Time => "time",
        }
    }
}

/// The remembered page state.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PageState {
    pub view: InsightsView,
    pub filter: InsightsFilter,
    pub scope: GraphScope,
    pub measure: Measure,
    pub network_colour: NetworkColour,
    pub map_colour: MapColour,
    pub dimension: SetDimension,
    /// Keys of the sets chosen in the Sets view (empty: the three largest).
    pub chosen_sets: Vec<String>,
    /// The thread drawn in the Symmetry view (by rank, largest first).
    pub thread: usize,
}

impl PageState {
    /// The state saved as `json`, or the default when missing or unreadable.
    pub fn from_json(json: Option<&str>) -> Self {
        json.and_then(|text| serde_json::from_str(text).ok()).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// What the report depends on (a change needs a recomputation).
    pub fn report_inputs(&self) -> (InsightsFilter, GraphScope) {
        (self.filter.clone(), self.scope)
    }
}

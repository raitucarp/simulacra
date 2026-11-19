//! Every graph metric of one graph in one value, plus rankings for the charts.

use super::betweenness::betweenness;
use super::bridges::{cut_structure, CutStructure};
use super::community::{communities, Communities};
use super::components::{components, Components};
use super::degree::{degrees, density, mean_degree, Degree};
use super::pagerank::pagerank;
use super::paths::diameter_estimate;
use super::triangles::{average_clustering, clustering};
use super::Graph;

/// Node-level and graph-level measures.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetworkMetrics {
    pub degrees: Vec<Degree>,
    pub pagerank: Vec<f64>,
    pub betweenness: Vec<f64>,
    /// Whether betweenness was estimated from sampled sources (large graphs).
    pub betweenness_sampled: bool,
    pub clustering: Vec<f64>,
    pub average_clustering: f64,
    pub density: f64,
    pub mean_degree: f64,
    pub components: Components,
    pub communities: Communities,
    pub cuts: CutStructure,
    /// Double-sweep lower bound of the largest component's diameter.
    pub diameter: usize,
}

/// A node measure to rank by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Measure {
    Degree,
    #[default]
    Influence,
    Betweenness,
    Clustering,
}

impl Measure {
    pub const ALL: [Self; 4] = [Self::Degree, Self::Influence, Self::Betweenness, Self::Clustering];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Degree => "degree",
            Self::Influence => "influence",
            Self::Betweenness => "betweenness",
            Self::Clustering => "clustering",
        }
    }
}

impl NetworkMetrics {
    /// Computes every measure of `graph`.
    pub fn compute(graph: &Graph) -> Self {
        let components = components(graph);
        let largest_start = components.of_node.iter().position(|&c| c == 0);
        let (betweenness, betweenness_sampled) = betweenness(graph);
        let clustering = clustering(graph);
        Self {
            degrees: degrees(graph),
            pagerank: pagerank(graph),
            betweenness,
            betweenness_sampled,
            average_clustering: average_clustering(graph, &clustering),
            clustering,
            density: density(graph),
            mean_degree: mean_degree(graph),
            communities: communities(graph),
            cuts: cut_structure(graph),
            diameter: largest_start.map_or(0, |start| diameter_estimate(graph, start)),
            components,
        }
    }

    /// The value of `measure` for `node`.
    pub fn value(&self, measure: Measure, node: usize) -> f64 {
        match measure {
            Measure::Degree => self.degrees.get(node).map_or(0.0, |d| d.total as f64),
            Measure::Influence => self.pagerank.get(node).copied().unwrap_or_default(),
            Measure::Betweenness => self.betweenness.get(node).copied().unwrap_or_default(),
            Measure::Clustering => self.clustering.get(node).copied().unwrap_or_default(),
        }
    }

    /// The `limit` nodes with the highest `measure` among those `keep` accepts (ties by index).
    pub fn top(&self, measure: Measure, limit: usize, keep: impl Fn(usize) -> bool) -> Vec<(usize, f64)> {
        let mut ranked: Vec<(usize, f64)> = (0..self.degrees.len()).filter(|&v| keep(v)).map(|v| (v, self.value(measure, v))).collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        ranked.truncate(limit);
        ranked
    }

    /// The highest value of `measure` (at least the smallest positive value).
    pub fn max(&self, measure: Measure) -> f64 {
        (0..self.degrees.len()).map(|v| self.value(measure, v)).fold(f64::MIN_POSITIVE, f64::max)
    }
}

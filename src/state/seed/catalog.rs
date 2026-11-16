//! Seed networks, trending topics, and grounding documents.

use super::parse_seed;
use crate::domain::{GroundingDocument, Network, NetworkId, TrendingTopic};
use serde::Deserialize;

const NETWORKS_ASSET: &str = "networks.json";
const NETWORKS_JSON: &str = include_str!("../../../assets/seed/networks.json");

const TRENDING_ASSET: &str = "trending.json";
const TRENDING_JSON: &str = include_str!("../../../assets/seed/trending.json");

const DOCUMENTS_ASSET: &str = "documents.json";
const DOCUMENTS_JSON: &str = include_str!("../../../assets/seed/documents.json");

/// Document record without derived fields; chunk count and timestamp are computed at seed time.
#[derive(Deserialize)]
struct DocumentSeed {
    id: String,
    network_id: NetworkId,
    title: String,
    content: String,
}

pub fn seed_networks() -> Vec<Network> {
    parse_seed(NETWORKS_ASSET, NETWORKS_JSON)
}

pub fn seed_trending() -> Vec<TrendingTopic> {
    parse_seed(TRENDING_ASSET, TRENDING_JSON)
}

pub fn seed_documents() -> Vec<GroundingDocument> {
    let seeds: Vec<DocumentSeed> = parse_seed(DOCUMENTS_ASSET, DOCUMENTS_JSON);
    seeds
        .into_iter()
        .map(|d| GroundingDocument::new(d.id, d.network_id, d.title, d.content))
        .collect()
}

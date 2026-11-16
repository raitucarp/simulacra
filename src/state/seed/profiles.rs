//! Seed human personas and per-network autonomous agents.

use super::parse_seed;
use crate::domain::{AgentConfig, NetworkId, Profile};
use serde::Deserialize;

const PERSONAS_ASSET: &str = "personas.json";
const PERSONAS_JSON: &str = include_str!("../../../assets/seed/personas.json");

/// Embeds one `assets/seed/agents/<name>.json` file per agent, in seed order.
macro_rules! agent_assets {
    ($($name:literal),* $(,)?) => {
        [$(($name, include_str!(concat!("../../../assets/seed/agents/", $name, ".json")))),*]
    };
}

/// (asset name, JSON) for every seeded agent, grouped by network.
const AGENT_ASSETS: [(&str, &str); 8] = agent_assets![
    // Network 1: `net_philosophy` (Epistemology & Mind)
    "socrates", "spinoza", "simone",
    // Network 2: `net_systems` (Distributed Systems & Rust)
    "elena", "marcus", "kenji",
    // Network 3: `net_creative` (Sci-Fi & Cybernetics)
    "ursula", "julian",
];

/// Controllable persona flavour, mapping to the matching `Profile` constructor.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum PersonaKind {
    Primary,
    Alt,
}

/// Controllable human persona; `config` lists only overrides of `AgentConfig::default()`.
#[derive(Deserialize)]
struct PersonaSeed {
    id: String,
    name: String,
    handle: String,
    kind: PersonaKind,
    network_id: Option<NetworkId>,
    config: AgentConfig,
}

/// Autonomous network member; omitted config fields take `AgentConfig::default()` values.
#[derive(Deserialize)]
struct AgentSeed {
    id: String,
    name: String,
    handle: String,
    persona_type: String,
    network_id: NetworkId,
    config: AgentConfig,
}

impl PersonaSeed {
    fn into_profile(self) -> Profile {
        let base = match self.kind {
            PersonaKind::Primary => Profile::new_primary(self.id, self.name, self.handle),
            PersonaKind::Alt => Profile::new_alt(self.id, self.name, self.handle),
        };
        let scoped = match self.network_id {
            Some(network_id) => base.with_network(network_id),
            None => base,
        };
        scoped.with_config(self.config)
    }
}

impl AgentSeed {
    fn into_profile(self) -> Profile {
        Profile::new_agent(self.id, self.name, self.handle, self.persona_type)
            .with_network(self.network_id)
            .with_config(self.config)
    }
}

pub fn seed_profiles() -> Vec<Profile> {
    let personas: Vec<PersonaSeed> = parse_seed(PERSONAS_ASSET, PERSONAS_JSON);
    let agents = AGENT_ASSETS
        .iter()
        .map(|(asset, json)| parse_seed::<AgentSeed>(asset, json).into_profile());
    personas
        .into_iter()
        .map(PersonaSeed::into_profile)
        .chain(agents)
        .collect()
}

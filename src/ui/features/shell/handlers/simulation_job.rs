//! Building one agent generation: who runs, what it writes (a reply or a new post) and
//! the engine job that carries it to the worker thread.

use super::super::view::AppView;
use crate::agent::runtime::EnginePlan;
use crate::agent::schedule::roll_percent;
use crate::agent::tools::ToolContext;
use crate::agent::{EngineJob, JobKind, PromptContext};
use crate::domain::post::TriggerMode;
use crate::domain::{NetworkId, Profile, ProfileId};

impl AppView {
    /// What a manual `mode` asks for: a reply to the network's top post (`None` while the
    /// network has no posts), or a new post from the seed with its vault document resolved.
    pub(super) fn trigger_job_kind(&self, mode: TriggerMode) -> Option<JobKind> {
        match mode {
            TriggerMode::Reply => self.state.current_network_posts().next().map(|p| JobKind::Reply(self.state.authored(p))),
            TriggerMode::NewPost { seed } => Some(JobKind::NewPost(seed.resolve(&self.state.documents))),
        }
    }

    /// The requested agent, or else the first active (then first) agent of the network.
    pub(super) fn simulation_agent(&self, agent_id: Option<&ProfileId>) -> Option<Profile> {
        if let Some(target_id) = agent_id {
            return self
                .state
                .user_profiles
                .iter()
                .find(|p| &p.id == target_id && p.is_agent())
                .cloned();
        }
        let agents = self.state.current_network_agents();
        agents
            .iter()
            .find(|p| p.config.is_active)
            .or_else(|| agents.first())
            .map(|p| (*p).clone())
    }

    /// The agent's engine resolved into a job: its plan, a tool snapshot when it has tools,
    /// and the image-chance roll.
    pub(super) fn engine_job(&self, agent: Profile, kind: JobKind, network: NetworkId) -> EngineJob {
        let engine = &agent.config.engine;
        let plan = EnginePlan::resolve(engine, &self.state.llm_settings, &self.state.credentials, self.state.media_dir());
        let tools = (!engine.tools.is_empty())
            .then(|| ToolContext::capture(&network, &self.state.posts, &self.state.user_profiles, &self.state.documents));
        let roll = roll_percent(self.scheduler.seed, agent.id.as_str(), self.scheduler.tick ^ self.state.posts.len() as u64);
        let context = PromptContext { beliefs: self.belief_dossier(&agent.id), ..self.prompt_context(&network) }; // sb F1
        EngineJob { kind, agent, network, context, plan, tools, roll }
    }

    /// The network's name and grounding document titles, for the agent's system prompt.
    pub(in crate::ui::features::shell) fn prompt_context(&self, network: &NetworkId) -> PromptContext {
        let network_name = self.state.networks.iter().find(|n| &n.id == network).map(|n| n.name.clone());
        let grounding_docs = self
            .state
            .documents
            .iter()
            .filter(|d| &d.network_id == network)
            .map(|d| d.title.clone())
            .collect();
        PromptContext { network_name: network_name.unwrap_or_default(), grounding_docs, ..PromptContext::default() }
    }
}

//! Domain entities and value objects for the Simulacra local social network.

pub mod trait_option;

pub mod activity;
pub mod agent_config;
pub mod document;
pub mod graph;
pub mod ids;
pub mod network;
pub mod portrait;
pub mod post;
pub mod profile;
pub mod thread;
pub mod topic;
pub mod traits;

pub use agent_config::{
    AgentConfig, AttachmentStyle, CapitalizationStyle, ClassicTemperament, CodeSwitchingStyle,
    CognitiveIntelligenceStyle, CognitivePace, ConflictHandlingStyle, DictionRegister, EmojiUsage,
    EnneagramType, EpistemicStance, GrammarStyle, HumanChronotype, HumanGender, HumorStyle,
    IdentityVariant, InstinctualVariant, LocusOfControl, MbtiType, MentionAttitude,
    MoralFoundation, PostingCadence, PrePostReasoningMode, PromptAssemblyStrategy,
    PunctuationHabit, SentenceLengthPreference,
};
pub use document::GroundingDocument;
pub use ids::{NetworkId, PostId, ProfileId};
pub use network::Network;
pub use post::{AuthoredPost, Post, MAX_POST_CHARS};
pub use profile::{Profile, ProfileRole};
pub use thread::{draft_issue, first_thread_problem, split_into_thread, DraftIssue, ThreadProblem};
pub use topic::TrendingTopic;
pub use trait_option::TraitOption;

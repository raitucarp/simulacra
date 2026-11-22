//! Domain entities and value objects for the Simulacra local social network.

pub mod trait_option;

pub mod activity;
pub mod agent_config;
pub mod archetype;
pub mod arena;
pub mod beliefs;
pub mod compose; // T1: free compose text to a tree of drafts
pub mod crypto; // sb X1: password-based seal shared by snapshots and encrypted posts
pub mod data; // sb F2: history, collections and bookmarks, export, snapshots
pub mod document;
pub mod emoji; // R3: emoji names, groups and skin tones
pub mod icons; // epic S: the downloadable icon library
pub mod embedding;
pub mod engine;
pub mod graph;
pub mod insights;
pub mod learning;
pub mod journal;
pub mod image_prompt;
pub mod media;
pub mod ids;
pub mod network;
pub mod palette;
pub mod preferences;
pub mod portrait;
pub mod post;
pub mod post_actions;
pub mod profile;
pub mod reaction; // R: emoji reactions and claps
pub mod readability; // sb F3
pub mod recommend;
pub mod review;
pub mod search_query; // Q2: search query language
pub mod spend; // sb P16: ask before spending
pub mod symbols; // S1: entry, state, signifiers, custom icons
pub mod thinking;
pub mod thread;
pub mod todo; // D2: to-do tree with spans
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
pub use archetype::ArchetypeClass;
pub use document::GroundingDocument;
pub use engine::EngineConfig;
pub use ids::{NetworkId, PostId, ProfileId};
pub use network::Network;
pub use palette::{Palette, PaletteCategory, PaletteScope};
pub use post::{AuthoredPost, Post, MAX_POST_CHARS};
pub use profile::{Profile, ProfileKind, ProfileRole, ANONYMOUS_PROFILE_ID};
pub use thread::{draft_issue, first_thread_problem, split_into_thread, DraftIssue, ThreadProblem};
pub use topic::TrendingTopic;
pub use trait_option::TraitOption;

//! 6. Prompt assembly & injection engine options.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// How the agent constructs its system and context prompt prior to publishing.
    pub enum PromptAssemblyStrategy in "assembly_strategy" {
        #[default]
        FullHumanSynthesis("full_human_synthesis", "Full Human Dossier Injection (All Parameters)"),
        PsychologyAndVoiceFocused("psychology_and_voice_focused", "Psychology, Intelligence & Linguistic Voice First"),
        CustomTemplate("custom_template", "Custom Template with {{Placeholders}} + Full Dossier"),
    }
}

trait_enum! {
    /// Pre-post internal reasoning approach before writing a post.
    pub enum PrePostReasoningMode in "pre_post_reasoning" {
        #[default]
        EmbodiedHumanPerspective("embodied_human_perspective", "Embodied Human Perspective (Inhabits Bio, Mood & Psychology)"),
        DialecticalNetworkWeaver("dialectical_network_weaver", "Dialectical Network Weaver (Connects Vault Docs & Wikilinks)"),
        DirectSpontaneous("direct_spontaneous", "Direct Spontaneous Microblog Thought"),
    }
}

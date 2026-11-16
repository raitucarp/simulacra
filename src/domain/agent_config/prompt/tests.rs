//! Golden tests pinning the exact prompt text for one seeded persona.
//!
//! The fixtures under `golden/` were captured from the pre-refactor implementation;
//! any byte of drift in prompt assembly fails these tests.

use crate::domain::traits::*;
use crate::domain::Profile;

const NETWORK: &str = "Agora";
const PARENT_NAME: &str = "Spinoza";
const PARENT_HANDLE: &str = "@spinoza";
const PARENT_POST_ID: &str = "post_7";
const PARENT_CONTENT: &str = "God or Nature.";
const EVERY_PLACEHOLDER_TEMPLATE: &str = "{{name}}|{{handle}}|{{description}}|{{gender}}|{{pronouns}}|{{bio}}|{{hobbies}}|{{mbti}}|{{enneagram}}\n{{identity}}\n{{psychology}}\n{{intelligence}}\n{{linguistics}}\n{{diction}}|{{grammar}}|{{language}}|{{typing_style}}\n{{schedule}}\n{{mention_attitude}}|{{network_context}}|{{directive}}";

fn seeded() -> Profile {
    let mut prof = Profile::new_agent("agent_golden", "Sokrates of Alopece", "sokrates", "Classical Philosopher");
    prof.config.mbti = MbtiType::Entp;
    prof.config.identity_variant = IdentityVariant::Turbulent;
    prof.config.enneagram = EnneagramType::Type6w7;
    prof.config.instinctual_variant = InstinctualVariant::SxSo;
    prof.config.diction_register = DictionRegister::SocraticIronic;
    prof.config.grammar_style = GrammarStyle::RelaxedConversational;
    prof
}

fn docs() -> Vec<String> {
    vec!["Tractatus".to_string(), "Ethics".to_string()]
}

/// Fixtures may be checked out with CRLF line endings on Windows.
fn golden(raw: &str) -> String {
    raw.replace("\r\n", "\n")
}

#[test]
fn system_prompt_full_human_synthesis_is_pinned() {
    let p = seeded();
    let prompt = p.config.assemble_system_prompt(&p, NETWORK, &docs());
    assert_eq!(prompt, golden(include_str!("golden/system_full.txt")));
}

#[test]
fn system_prompt_voice_focused_is_pinned() {
    let mut p = seeded();
    p.config.assembly_strategy = PromptAssemblyStrategy::PsychologyAndVoiceFocused;
    p.config.pre_post_reasoning = PrePostReasoningMode::DialecticalNetworkWeaver;
    let prompt = p.config.assemble_system_prompt(&p, NETWORK, &docs());
    assert_eq!(prompt, golden(include_str!("golden/system_voice.txt")));
}

#[test]
fn system_prompt_custom_template_is_pinned() {
    let mut p = seeded();
    p.config.assembly_strategy = PromptAssemblyStrategy::CustomTemplate;
    p.config.pre_post_reasoning = PrePostReasoningMode::DirectSpontaneous;
    p.config.prompt_template = EVERY_PLACEHOLDER_TEMPLATE.to_string();
    p.config.inject_network_grounding = false;
    p.config.inject_intelligence_profile = false;
    let prompt = p.config.assemble_system_prompt(&p, NETWORK, &docs());
    assert_eq!(prompt, golden(include_str!("golden/system_template.txt")));
}

#[test]
fn user_prompts_are_pinned() {
    let p = seeded();
    let mentioned = p.config.assemble_user_prompt(&p, PARENT_NAME, PARENT_HANDLE, PARENT_POST_ID, PARENT_CONTENT, true);
    assert_eq!(mentioned, golden(include_str!("golden/user_mentioned.txt")));
    let stream = p.config.assemble_user_prompt(&p, PARENT_NAME, PARENT_HANDLE, PARENT_POST_ID, PARENT_CONTENT, false);
    assert_eq!(stream, golden(include_str!("golden/user_stream.txt")));
}

#[test]
fn every_listed_template_placeholder_is_expanded() {
    let mut p = seeded();
    p.config.assembly_strategy = PromptAssemblyStrategy::CustomTemplate;
    p.config.prompt_template = super::TEMPLATE_PLACEHOLDERS.join("|");
    let prompt = p.config.assemble_system_prompt(&p, NETWORK, &docs());
    for placeholder in super::TEMPLATE_PLACEHOLDERS {
        assert!(!prompt.contains(placeholder), "{placeholder} was not expanded");
        assert!(placeholder == super::ALL_AGENT_INFO_PLACEHOLDER || EVERY_PLACEHOLDER_TEMPLATE.contains(placeholder));
    }
}

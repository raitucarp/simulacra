use super::*;
use crate::domain::Profile;

#[test]
fn test_agent_config_default_and_serde_roundtrip() {
    let cfg = AgentConfig::default();
    assert_eq!(cfg.mbti, MbtiType::Intj);
    assert_eq!(cfg.mbti.cognitive_functions(), "Ni · Te · Fi · Se");
    assert_eq!(cfg.mbti_full_code(), "INTJ-A");

    let json = serde_json::to_string(&cfg).expect("serialize AgentConfig");
    let loaded: AgentConfig = serde_json::from_str(&json).expect("deserialize AgentConfig");
    assert_eq!(cfg, loaded);
}

#[test]
fn test_partial_legacy_json_fills_defaults() {
    let json = r#"{"gender":"NonBinary","mbti":"Enfp","enneagram":"Type7w8","age":51}"#;
    let cfg: AgentConfig = serde_json::from_str(json).expect("deserialize partial config");
    assert_eq!(cfg.gender, HumanGender::NonBinary);
    assert_eq!(cfg.mbti, MbtiType::Enfp);
    assert_eq!(cfg.enneagram, EnneagramType::Type7w8);
    assert_eq!(cfg.age, 51);
    assert_eq!(cfg.openness, AgentConfig::default().openness);
    assert_eq!(cfg.injected_section_count(), 6);
}

#[test]
fn test_assemble_system_prompt_injects_all_agent_info() {
    let mut prof = Profile::new_agent("agent_1", "Sokrates of Alopece", "@sokrates", "Classical Philosopher");
    prof.config.gender = HumanGender::Male;
    prof.config.mbti = MbtiType::Entp;
    prof.config.enneagram = EnneagramType::Type5w4;
    prof.config.hobbies = "Barefoot walks in the Agora, questioning artisans".to_string();
    prof.config.primary_language = "Indonesian".to_string();
    prof.config.grammar_style = GrammarStyle::RelaxedConversational;
    prof.config.diction_register = DictionRegister::SocraticIronic;

    let docs = vec!["Tractatus Logico-Philosophicus".to_string()];
    let prompt = prof.config.assemble_system_prompt(&prof, "Epistemology & Mind", &docs);

    assert!(prompt.contains("Sokrates of Alopece"));
    assert!(prompt.contains("@sokrates"));
    assert!(prompt.contains("ENTP"));
    assert!(prompt.contains("Ne · Ti · Fe · Si"));
    assert!(prompt.contains("5w4"));
    assert!(prompt.contains("Barefoot walks in the Agora"));
    assert!(prompt.contains("Indonesian"));
    assert!(prompt.contains("Relaxed Everyday Conversational"));
    assert!(prompt.contains("Tractatus Logico-Philosophicus"));
}

#[test]
fn test_custom_template_placeholder_expansion() {
    let mut prof = Profile::new_agent("agent_2", "Elena Rostova", "@elena_rostova", "Systems Architect");
    prof.config.assembly_strategy = PromptAssemblyStrategy::CustomTemplate;
    prof.config.prompt_template = "Agent: {{name}} | MBTI: {{mbti}} | Lang: {{language}} | Hobbies: {{hobbies}}".to_string();
    prof.config.mbti = MbtiType::Istp;
    prof.config.primary_language = "English".to_string();
    prof.config.hobbies = "Kernel profiling & lock-free queues".to_string();

    let prompt = prof.config.assemble_system_prompt(&prof, "Distributed Systems", &[]);
    assert!(prompt.contains("Agent: Elena Rostova | MBTI: ISTP | Lang: English | Hobbies: Kernel profiling & lock-free queues"));
}

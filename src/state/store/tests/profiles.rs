use crate::domain::NetworkId;
use crate::state::store::AppState;
use crate::storage::Database;
use crate::storage::Vault;

#[test]
fn test_agents_are_human_named_and_scoped_per_network() {
    let mut state = AppState::new_in_memory();

    // 1. Verify NO profile uses bot/GPT/Node naming
    for prof in &state.user_profiles {
        assert!(!prof.name.contains("GPT"), "Agent must not contain GPT: {}", prof.name);
        assert!(!prof.name.contains("Bot"), "Agent must not contain Bot: {}", prof.name);
        assert!(!prof.name.contains("-Node"), "Agent must not contain -Node: {}", prof.name);
        assert!(!prof.handle.contains("_ai"), "Handle must not contain _ai: {}", prof.handle);
        assert!(!prof.handle.contains("_bot"), "Handle must not contain _bot: {}", prof.handle);
    }

    // 2. Network 1: `net_philosophy` has Sokrates, Baruch de Spinoza, Simone Weil
    assert_eq!(state.active_network_id.as_str(), "net_philosophy");
    let phil_agents: Vec<String> = state
        .current_network_agents()
        .into_iter()
        .map(|p| p.handle.clone())
        .collect();
    assert_eq!(phil_agents, vec!["@sokrates", "@baruch_spinoza", "@simone_weil"]);

    // 3. Switch to Network 2: `net_systems` -> Elena Rostova, Marcus Vance, Kenji Takahashi
    state.switch_network(&NetworkId::new("net_systems")).unwrap();
    let sys_agents: Vec<String> = state
        .current_network_agents()
        .into_iter()
        .map(|p| p.handle.clone())
        .collect();
    assert_eq!(sys_agents, vec!["@elena_rostova", "@marcus_vance", "@kenji_t"]);

    // 4. Switch to Network 3: `net_creative` -> Ursula K. Vance, Julian Sterling
    state.switch_network(&NetworkId::new("net_creative")).unwrap();
    let creative_agents: Vec<String> = state
        .current_network_agents()
        .into_iter()
        .map(|p| p.handle.clone())
        .collect();
    assert_eq!(creative_agents, vec!["@ursula_v", "@julian_sterling"]);
}

#[test]
fn test_create_custom_agent_and_persona_lab_separation() {
    let db = Vault::spawn(Database::in_memory().expect("in-memory db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(db.clone()));

    // Create a custom human-like agent in `net_philosophy`
    let new_agent_id = state
        .create_network_agent("Ki Hajar Dewantara", "kihajar", "Philosopher of Education")
        .expect("create custom agent");

    // Verify it is in `net_philosophy` agents but NOT in `net_systems` agents
    assert!(state.current_network_agents().iter().any(|p| p.id == new_agent_id));
    assert!(!state.current_network_personas().iter().any(|p| p.id == new_agent_id));

    state.switch_network(&NetworkId::new("net_systems")).unwrap();
    assert!(!state.current_network_agents().iter().any(|p| p.id == new_agent_id));

    // Switch back to `net_philosophy` and customize the agent's full psychological & linguistic profile
    state.switch_network(&NetworkId::new("net_philosophy")).unwrap();
    let mut agent_prof = state
        .user_profiles
        .iter()
        .find(|p| p.id == new_agent_id)
        .cloned()
        .unwrap();
    agent_prof.avatar_path = Some("C:/avatars/kihajar.png".to_string());
    agent_prof.config.mbti = crate::domain::MbtiType::Enfj;
    agent_prof.config.enneagram = crate::domain::EnneagramType::Type1w2;
    agent_prof.config.primary_language = "Indonesian".to_string();
    agent_prof.config.hobbies = "Gamelan orchestration, educational philosophy, journalism".to_string();
    state.update_profile(agent_prof).unwrap();

    // Create a custom controllable human persona in Persona Lab and switch to it
    let new_persona_id = state
        .create_human_persona("Tan Malaka", "@tan_malaka", "Madilog Dialectician")
        .expect("create human persona");
    assert!(state.current_network_personas().iter().any(|p| p.id == new_persona_id));
    assert!(!state.current_network_agents().iter().any(|p| p.id == new_persona_id));

    state.switch_profile(&new_persona_id).unwrap();
    assert_eq!(state.active_profile_id, new_persona_id);

    // Cycle active persona and verify it never selects an autonomous agent
    for _ in 0..5 {
        let cycled = state.cycle_active_persona().expect("cycled persona");
        let prof = state.user_profiles.iter().find(|p| p.id == cycled).unwrap();
        assert!(prof.is_human_persona(), "Cycle must only select controllable human personas");
    }

    // Reload from SQLite and verify full persistence of custom agent & persona
    let reloaded = AppState::load_from_db(&db).expect("reload from db");
    let loaded_agent = reloaded
        .user_profiles
        .iter()
        .find(|p| p.id == new_agent_id)
        .expect("custom agent persisted");
    assert_eq!(loaded_agent.name, "Ki Hajar Dewantara");
    assert_eq!(loaded_agent.handle, "@kihajar");
    assert_eq!(loaded_agent.avatar_path.as_deref(), Some("C:/avatars/kihajar.png"));
    assert_eq!(loaded_agent.config.mbti, crate::domain::MbtiType::Enfj);
    assert_eq!(loaded_agent.config.enneagram, crate::domain::EnneagramType::Type1w2);
    assert_eq!(loaded_agent.config.primary_language, "Indonesian");
    assert!(loaded_agent.config.hobbies.contains("Gamelan orchestration"));
}

#[test]
fn authors_resolve_by_id_and_survive_profile_edits_and_deletion() {
    let mut state = AppState::new_in_memory();
    // A custom persona (deleted seed profiles are remembered and not re-seeded either).
    let persona_id = state.create_human_persona("Ephemeral", "@ephemeral", "temp").unwrap();
    let agent = state.find_author(&persona_id).cloned().expect("created persona");
    state.switch_profile(&agent.id).expect("switch to persona");
    let post_id = state.submit_post("authored by a persona").unwrap();
    let post = state.get_post(&post_id).unwrap().clone();
    assert_eq!(state.author_of(&post).name, agent.name);

    let mut renamed = agent.clone();
    renamed.name = "Renamed Thinker".to_string();
    state.update_profile(renamed).unwrap();
    assert_eq!(state.author_of(&post).name, "Renamed Thinker", "edits apply without rewriting posts");

    state.delete_profile(&agent.id).unwrap();
    assert_eq!(state.author_of(&post).name, "Renamed Thinker", "deleted authors stay archived");
    let db = state.db.clone().expect("in-memory db");
    let reloaded = AppState::load_from_db(&db).expect("reload");
    let reloaded_post = reloaded.get_post(&post_id).expect("post persisted");
    assert_eq!(reloaded.author_of(reloaded_post).name, "Renamed Thinker");
}

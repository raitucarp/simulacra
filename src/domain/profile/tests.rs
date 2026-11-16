use super::*;

#[test]
fn constructors_set_role_initial_description_and_network() {
    let primary = Profile::new_primary("u1", "architect", "@arch");
    assert_eq!(primary.avatar_initial, "A");
    assert_eq!(primary.role, ProfileRole::UserPrimary);
    assert_eq!(primary.config.description, "Primary Architect");
    assert_eq!(primary.network_id, None);

    let alt = Profile::new_alt("u2", "", "alt");
    assert_eq!(alt.avatar_initial, "A");
    assert_eq!(alt.config.description, "Alter-Ego Persona");

    let agent = Profile::new_agent("a1", "", "@bot", "Stoic");
    assert_eq!(agent.avatar_initial, "S");
    assert_eq!(agent.role, ProfileRole::Agent { persona_type: "Stoic".to_string() });
    assert_eq!(agent.config.description, "Stoic");
    assert_eq!(agent.network_id, Some(NetworkId::new("net_philosophy")));
    assert_eq!(Profile::new_primary("u3", "", "x").avatar_initial, "U");
}

#[test]
fn role_labels_and_handles() {
    assert_eq!(Profile::new_primary("u1", "A", "a").role_label(), "YOU");
    assert_eq!(Profile::new_alt("u2", "B", "b").role_label(), "PERSONA");
    assert_eq!(Profile::new_agent("a1", "C", "c", "").role_label(), "MEMBER");
    assert_eq!(Profile::new_agent("a2", "D", "d", "Cynic").role_label(), "Cynic");
    assert_eq!(Profile::new_primary("u1", "A", "plain").display_handle(), "@plain");
    assert_eq!(Profile::new_primary("u1", "A", "@at").display_handle(), "@at");
}

#[test]
fn network_membership_rules() {
    let net = NetworkId::new("net_other");
    let default_net = NetworkId::new("net_philosophy");
    let mut agent = Profile::new_agent("a1", "C", "c", "x");
    assert!(!agent.belongs_to_network(&net));
    agent.network_id = None;
    assert!(agent.belongs_to_network(&default_net));
    assert!(!agent.belongs_to_network(&net));
    let persona = Profile::new_alt("u2", "B", "b");
    assert!(persona.belongs_to_network(&net));
    assert!(!persona.clone().with_network("net_x").belongs_to_network(&net));
}

#[test]
fn builders_and_sync() {
    let p = Profile::new_alt("u2", "B", "b").with_avatar_path("   ");
    assert_eq!(p.avatar_path, None);
    let cfg = AgentConfig { description: "Renamed".to_string(), ..AgentConfig::default() };
    let mut agent = Profile::new_agent("a1", "zed", "z", "Old").with_config(cfg);
    assert_eq!(agent.role_label(), "Renamed");
    agent.avatar_initial.clear();
    agent.config.description = "Synced".to_string();
    agent.sync_derived_fields();
    assert_eq!(agent.avatar_initial, "Z");
    assert_eq!(agent.role_label(), "Synced");
}

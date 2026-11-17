use crate::domain::preferences::*;

fn legacy(scheduler: Option<&str>) -> LegacyValues {
    LegacyValues { scheduler_enabled: scheduler.map(str::to_string) }
}

#[test]
fn legacy_scheduler_switch_moves_into_agents() {
    let on = migrate_legacy(Preferences::default(), &legacy(Some("true")));
    assert!(on.agents.autonomous_posting);
    let off = migrate_legacy(Preferences::default(), &legacy(Some(" false ")));
    assert!(!off.agents.autonomous_posting);
}

#[test]
fn missing_or_unreadable_legacy_values_keep_the_base() {
    assert_eq!(migrate_legacy(Preferences::default(), &legacy(None)), Preferences::default());
    assert_eq!(migrate_legacy(Preferences::default(), &legacy(Some("maybe"))), Preferences::default());
    let base = Preferences { agents: AgentPrefs { autonomous_posting: true, ..AgentPrefs::default() }, ..Preferences::default() };
    assert!(migrate_legacy(base, &legacy(Some("yes please"))).agents.autonomous_posting);
}

#[test]
fn migration_leaves_other_categories_alone() {
    let mut base = Preferences::default();
    base.learning.daily_review_limit = 500;
    base.general.startup_screen = StartupScreen::MindGraph;
    let migrated = migrate_legacy(base.clone(), &legacy(Some("true")));
    assert_eq!(migrated.learning, base.learning);
    assert_eq!(migrated.general, base.general);
}

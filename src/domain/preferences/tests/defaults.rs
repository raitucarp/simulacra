use crate::domain::preferences::*;
use crate::domain::MAX_POST_CHARS;
use crate::i18n::Language;

#[test]
fn defaults_are_safe_and_already_valid() {
    let prefs = Preferences::default();
    assert!(!prefs.agents.autonomous_posting, "never surprise-post");
    assert!(!prefs.privacy.telemetry, "telemetry is off");
    assert!(prefs.ai.ask_before_spending);
    assert_eq!(prefs.general.language, Language::English);
    assert_eq!(prefs.general.startup_screen, StartupScreen::Timeline);
    assert_eq!(prefs.writing.post_length_limit as usize, MAX_POST_CHARS);
    assert_eq!(prefs.embeddings.provider, EmbeddingProvider::Gemini);
    assert_eq!(prefs.embeddings.model, DEFAULT_EMBEDDING_MODEL);
    assert_eq!(prefs.agents.max_concurrent, 1);
    assert_eq!(prefs.clone().validated(), prefs, "defaults survive validation unchanged");
}

#[test]
fn numbers_clamp_into_their_bounds() {
    let mut prefs = Preferences::default();
    prefs.writing.post_length_limit = 1;
    prefs.writing.autosave_seconds = 99_999;
    prefs.ai.max_retries = 50;
    prefs.agents.quiet_start = 30;
    prefs.agents.max_concurrent = 0;
    prefs.learning.desired_retention = 100;
    prefs.import.max_file_mb = 0;
    let clamped = prefs.validated();
    assert_eq!(clamped.writing.post_length_limit as i64, POST_LENGTH.min);
    assert_eq!(clamped.writing.autosave_seconds as i64, AUTOSAVE_SECONDS.max);
    assert_eq!(clamped.ai.max_retries as i64, MAX_RETRIES.max);
    assert_eq!(clamped.agents.quiet_start, 23);
    assert_eq!(clamped.agents.max_concurrent as i64, MAX_CONCURRENT.min);
    assert_eq!(clamped.learning.desired_retention as i64, DESIRED_RETENTION.max);
    assert_eq!(clamped.import.max_file_mb as i64, MAX_FILE_MB.min);
    assert_eq!(clamped.clone().validated(), clamped, "validation is idempotent");
}

#[test]
fn dependent_values_follow_their_partner() {
    let mut prefs = Preferences::default();
    prefs.embeddings.chunk_size = 256;
    prefs.embeddings.chunk_overlap = 1000;
    prefs.learning.daily_review_limit = 50;
    prefs.learning.new_cards_per_day = 400;
    let clamped = prefs.validated();
    assert_eq!(clamped.embeddings.chunk_overlap, 128, "overlap is at most half a chunk");
    assert_eq!(clamped.learning.new_cards_per_day, 50, "new cards never exceed the review limit");
}

#[test]
fn text_and_telemetry_are_normalised() {
    let mut prefs = Preferences::default();
    prefs.embeddings.model = "   ".into();
    prefs.images.style_prefix = format!("  {}  ", "x".repeat(500));
    prefs.general.default_persona = "  user_main ".into();
    prefs.privacy.telemetry = true;
    let clean = prefs.validated();
    assert_eq!(clean.embeddings.model, DEFAULT_EMBEDDING_MODEL);
    assert_eq!(clean.images.style_prefix.chars().count(), 200);
    assert_eq!(clean.general.default_persona, "user_main");
    assert!(!clean.privacy.telemetry, "telemetry can never be switched on");
}

#[test]
fn quiet_hours_wrap_past_midnight() {
    let night = AgentPrefs { quiet_hours: true, quiet_start: 22, quiet_end: 7, ..AgentPrefs::default() };
    assert!(night.is_quiet_at(23) && night.is_quiet_at(0) && night.is_quiet_at(6));
    assert!(!night.is_quiet_at(7) && !night.is_quiet_at(12) && !night.is_quiet_at(21));
    let lunch = AgentPrefs { quiet_start: 12, quiet_end: 14, ..night.clone() };
    assert!(lunch.is_quiet_at(12) && lunch.is_quiet_at(13) && !lunch.is_quiet_at(14));
    assert!(!AgentPrefs { quiet_hours: false, ..night.clone() }.is_quiet_at(23), "switched off");
    assert!(!AgentPrefs { quiet_start: 5, quiet_end: 5, ..night }.is_quiet_at(5), "empty window");
}

#[test]
fn toast_kinds_respect_the_master_switch() {
    let prefs = NotificationPrefs::default();
    assert!(prefs.allows(ToastKind::Confirmation) && prefs.allows(ToastKind::AiFailure));
    assert!(!prefs.allows(ToastKind::AgentActivity));
    let muted = NotificationPrefs { toasts: false, ..prefs.clone() };
    assert!(!muted.allows(ToastKind::Confirmation) && !muted.allows(ToastKind::AiFailure));
    let quiet_ai = NotificationPrefs { ai_failures: false, ..prefs };
    assert!(!quiet_ai.allows(ToastKind::AiFailure) && quiet_ai.allows(ToastKind::Confirmation));
}

#[test]
fn bounds_step_and_clamp() {
    let bounds = Bounds::new(10, 50, 10);
    assert_eq!(bounds.stepped(10, -1), 10);
    assert_eq!(bounds.stepped(40, 1), 50);
    assert_eq!(bounds.stepped(50, 1), 50);
    assert_eq!(bounds.stepped(20, 2), 40);
    assert_eq!(bounds.clamp_u32(u32::MAX), 50);
}

use crate::domain::preferences::*;
use crate::i18n::Language;

fn customised() -> Preferences {
    let mut prefs = Preferences::default();
    prefs.general.language = Language::ChineseSimplified;
    prefs.general.date_format = DateFormat::DayMonthYear;
    prefs.general.default_persona = "user_main".into();
    prefs.appearance.reduced_motion = true;
    prefs.writing.default_identity = PostIdentity::Pseudonym;
    prefs.ai.daily_token_cap = 100_000;
    prefs.embeddings.model = "gemini-embedding-001".into();
    prefs.images.size = ImageAspect::Landscape;
    prefs.agents.autonomous_posting = true;
    prefs.learning.desired_retention = 85;
    prefs.import.segmentation = ImportSegmentation::Chapter;
    prefs.notifications.ai_failures = false;
    prefs.advanced.log_level = LogLevel::Debug;
    prefs.validated()
}

#[test]
fn preferences_round_trip_through_json() {
    let prefs = customised();
    let json = serde_json::to_string(&prefs).expect("serialize");
    let back: Preferences = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, prefs);
}

#[test]
fn choices_and_language_use_stable_wire_keys() {
    let json = serde_json::to_value(customised()).expect("serialize");
    assert_eq!(json["general"]["language"], "zh-CN");
    assert_eq!(json["general"]["date_format"], "day_month_year");
    assert_eq!(json["general"]["time_format"], "24h");
    assert_eq!(json["writing"]["default_identity"], "pseudonym");
    assert_eq!(json["advanced"]["log_level"], "debug");
}

#[test]
fn missing_fields_take_defaults_and_unknown_keys_fall_back() {
    let partial: GeneralPrefs = serde_json::from_str(r#"{"language":"ja","date_format":"stardate"}"#).expect("lenient");
    assert_eq!(partial.language, Language::Japanese);
    assert_eq!(partial.date_format, DateFormat::default(), "unknown option reads as the default");
    assert_eq!(partial.startup_screen, StartupScreen::default(), "missing field reads as the default");
    let empty: Preferences = serde_json::from_str("{}").expect("empty object");
    assert_eq!(empty, Preferences::default());
    let odd: GeneralPrefs = serde_json::from_str(r#"{"language":"tlh"}"#).expect("unknown language");
    assert_eq!(odd.language, Language::English);
}

#[test]
fn every_choice_key_parses_back() {
    fn check<C: Choice + std::fmt::Debug>() {
        for key in C::KEYS {
            let option = C::from_key(key).expect("known key");
            assert_eq!(option.key(), *key);
        }
        assert_eq!(C::from_key("no-such-option"), None);
    }
    check::<StartupScreen>();
    check::<DateFormat>();
    check::<TimeFormat>();
    check::<NumberFormat>();
    check::<FirstWeekday>();
    check::<ReadingWidth>();
    check::<PostIdentity>();
    check::<WikilinkClick>();
    check::<HashtagClick>();
    check::<ImportSegmentation>();
    check::<EmbeddingProvider>();
    check::<ImageEngineChoice>();
    check::<ImageAspect>();
    check::<LogLevel>();
    check::<Language>();
}

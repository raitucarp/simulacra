use super::stage::{fade_delay, millis, warm_up_done, SplashStage};
use std::time::Duration;

#[test]
fn stages_advance_in_order_and_stop_at_ready() {
    let mut stage = SplashStage::OpeningVault;
    let mut seen = vec![stage];
    while stage != SplashStage::Ready {
        stage = stage.next();
        seen.push(stage);
    }
    assert_eq!(seen, SplashStage::ALL.to_vec());
    assert_eq!(SplashStage::Ready.next(), SplashStage::Ready);
}

#[test]
fn progress_grows_with_every_stage_and_fills_at_ready() {
    let progress: Vec<f32> = SplashStage::ALL.iter().map(|stage| stage.progress()).collect();
    assert!(progress[0] > 0.0, "the first stage already shows some progress");
    assert!(progress.windows(2).all(|pair| pair[0] < pair[1]), "{progress:?}");
    assert_eq!(SplashStage::Ready.progress(), 1.0);
}

#[test]
fn every_stage_has_its_own_label() {
    let mut keys: Vec<&str> = SplashStage::ALL.iter().map(|stage| stage.label_key()).collect();
    keys.dedup();
    assert_eq!(keys.len(), SplashStage::ALL.len());
    assert!(keys.iter().all(|key| key.starts_with("splash.stage.")));
}

#[test]
fn splash_waits_out_its_minimum_time() {
    let minimum = Duration::from_millis(900);
    assert_eq!(fade_delay(Duration::from_millis(200), minimum), Duration::from_millis(700));
    assert_eq!(fade_delay(Duration::from_millis(900), minimum), Duration::ZERO);
    assert_eq!(fade_delay(Duration::from_secs(5), minimum), Duration::ZERO);
}

#[test]
fn token_milliseconds_convert_to_durations() {
    assert_eq!(millis(280.0), Duration::from_millis(280));
    assert_eq!(millis(0.0), Duration::ZERO);
    assert_eq!(millis(-5.0), Duration::ZERO);
    assert_eq!(millis(f32::NAN), Duration::ZERO);
}

#[test]
fn default_tokens_keep_the_splash_brief() {
    let theme = crate::theme::ThemeTokens::default();
    let minimum = millis(theme.splash_min_duration_ms);
    assert!(minimum > Duration::ZERO && minimum < Duration::from_secs(3), "{minimum:?}");
    assert!(millis(theme.splash_fade_duration_ms) < minimum);
}

#[test]
fn warm_up_ends_when_everything_is_ready_or_time_runs_out() {
    let limit = Duration::from_millis(1500);
    assert!(warm_up_done(0, Duration::ZERO, limit));
    assert!(!warm_up_done(3, Duration::from_millis(200), limit));
    assert!(warm_up_done(3, limit, limit), "a slow image never blocks startup");
    assert!(SplashStage::WarmingUp.progress() < SplashStage::Ready.progress());
}

#[test]
fn the_splash_window_is_smaller_than_the_main_windows_restore_size() {
    let theme = crate::theme::ThemeTokens::default();
    assert!(theme.splash_window_width < theme.window_default_width);
    assert!(theme.splash_window_height < theme.window_default_height);
    assert!(theme.window_min_width <= theme.window_default_width && theme.window_min_height <= theme.window_default_height);
    assert!(theme.splash_progress_width < theme.splash_window_width);
    assert!(millis(theme.splash_warm_up_limit_ms) > Duration::ZERO);
}

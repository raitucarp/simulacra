//! Pure sequencing of the splash screen: the loading stages in order, their labels and
//! progress, and when the fade into the shell may start.

use std::time::Duration;

/// What the splash says it is doing; each stage's work runs right after it is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplashStage {
    /// Starting the vault worker on the opened database.
    OpeningVault,
    /// Reading networks, profiles, posts and settings.
    LoadingNetworks,
    /// Building the shell: feeds, the graph, the agents' scheduler.
    WakingAgents,
    /// Everything is loaded; the splash waits out its minimum time, then fades.
    Ready,
}

impl SplashStage {
    /// Every stage, in order.
    pub const ALL: [Self; 4] = [Self::OpeningVault, Self::LoadingNetworks, Self::WakingAgents, Self::Ready];

    /// The stage after this one (`Ready` stays `Ready`).
    pub fn next(self) -> Self {
        match self {
            Self::OpeningVault => Self::LoadingNetworks,
            Self::LoadingNetworks => Self::WakingAgents,
            Self::WakingAgents | Self::Ready => Self::Ready,
        }
    }

    /// Translation key of the stage label.
    pub const fn label_key(self) -> &'static str {
        match self {
            Self::OpeningVault => "splash.stage.opening_vault",
            Self::LoadingNetworks => "splash.stage.loading_networks",
            Self::WakingAgents => "splash.stage.waking_agents",
            Self::Ready => "splash.stage.ready",
        }
    }

    /// Share of the progress line filled while this stage is shown (0–1, never empty).
    pub fn progress(self) -> f32 {
        let position = Self::ALL.iter().position(|stage| *stage == self).unwrap_or_default();
        (position + 1) as f32 / Self::ALL.len() as f32
    }
}

/// How long the splash still stays up once loading is done, so it is shown for at least
/// `minimum` overall.
pub fn fade_delay(elapsed: Duration, minimum: Duration) -> Duration {
    minimum.saturating_sub(elapsed)
}

/// A token duration in milliseconds as a [`Duration`] (negative or non-finite values are zero).
pub fn millis(ms: f32) -> Duration {
    if ms.is_finite() && ms > 0.0 {
        Duration::from_millis(ms.round() as u64)
    } else {
        Duration::ZERO
    }
}

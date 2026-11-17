//! Splash visuals: the brand mark, the app name and tagline, Mnemo beside the progress line
//! and stage label, and the version. The whole window is a drag area (it has no title bar).

use super::{SplashPhase, SplashView};
use crate::theme::ThemeTokens;
use crate::ui::illustration::{GuidePose, Illustration, IllustrationKind};
use gpui_kit::*;
use rust_i18n::t;

/// Brand mark template (trait-card canvas, see `assets/illustrations/STYLE.md`).
const BRAND_MARK: &str = "illustrations/splash/brand.svg";
/// Element id of the splash content (also keys its fade animation).
const SPLASH_ID: &str = "app-splash";
/// Version shown under the splash.
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

impl SplashView {
    /// The splash content, fading out once the main window is ready.
    pub(super) fn render_splash(&self) -> AnyElement {
        let theme = &self.theme;
        let splash = div()
            .id(SPLASH_ID)
            .absolute()
            .inset_0()
            .flex()
            .flex_col()
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .window_control_area(WindowControlArea::Drag)
            .child(self.center(theme))
            .child(version(theme));
        if self.phase != SplashPhase::Fading {
            return splash.into_any_element();
        }
        let fade = Animation::new(super::stage::millis(theme.splash_fade_duration_ms));
        splash.with_animation(SPLASH_ID, fade, |splash, delta| splash.opacity(1.0 - delta)).into_any_element()
    }

    /// Brand mark, name, tagline and the progress block, centered.
    fn center(&self, theme: &ThemeTokens) -> Div {
        div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(theme.spacing_lg)
            .child(Illustration::asset(BRAND_MARK, IllustrationKind::TraitCard).width(theme.splash_brand_width))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(theme.spacing_xs)
                    .child(div().text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(t!("app.title").to_string()))
                    .child(div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(t!("splash.tagline").to_string())),
            )
            .child(self.progress(theme))
    }

    /// Mnemo reading beside the progress line and the current stage label.
    fn progress(&self, theme: &ThemeTokens) -> Div {
        let stage = self.stage();
        let track = div()
            .w(theme.splash_progress_width)
            .h(theme.splash_progress_height)
            .rounded(theme.radius_full)
            .bg(theme.border)
            .child(div().h_full().w(theme.splash_progress_width * stage.progress()).rounded(theme.radius_full).bg(theme.accent_brand));
        let label = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!(stage.label_key()).to_string());
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_md)
            .child(Illustration::guide(GuidePose::Read).width(theme.splash_guide_size))
            .child(div().flex().flex_col().gap(theme.spacing_sm).child(track).child(label))
    }
}

/// The app version, bottom center.
fn version(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex()
        .justify_center()
        .pb(theme.spacing_xl)
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(t!("splash.version", version = APP_VERSION).to_string())
}

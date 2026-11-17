//! The first-run welcome: Mnemo greets a fresh vault, three short cards say what Simulacra
//! is, and the primary action opens the Agent Forge to create the first thinker. Skipping
//! (or the scrim) dismisses it; both are remembered by the host.

use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{GuidePose, Illustration};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const BACKDROP_ID: &str = "onboarding-backdrop";
const DIALOG_ID: &str = "onboarding-dialog";
const START_ID: &str = "onboarding-start";
const SKIP_ID: &str = "onboarding-skip";
const TITLE_KEY: &str = "onboarding.title";
const SUBTITLE_KEY: &str = "onboarding.subtitle";
const REPLAY_HINT_KEY: &str = "onboarding.replay_hint";
const SKIP_KEY: &str = "onboarding.skip";
const START_KEY: &str = "onboarding.start";
/// (icon, title key, body key) of each card, in reading order.
const CARDS: [(LucideIcon, &str, &str); 3] = [
    (LucideIcon::Sparkles, "onboarding.what_title", "onboarding.what_body"),
    (LucideIcon::Bot, "onboarding.life_title", "onboarding.life_body"),
    (LucideIcon::Plus, "onboarding.create_title", "onboarding.create_body"),
];

/// A shared no-argument callback.
pub type OnboardingAction = Rc<dyn Fn(&mut Window, &mut App)>;

/// The welcome overlay; `on_start` opens the forge, `on_skip` dismisses it.
#[derive(IntoElement)]
pub struct WelcomeOverlay {
    theme: ThemeTokens,
    on_start: OnboardingAction,
    on_skip: OnboardingAction,
}

impl WelcomeOverlay {
    pub fn new(theme: &ThemeTokens, on_start: OnboardingAction, on_skip: OnboardingAction) -> Self {
        Self { theme: theme.clone(), on_start, on_skip }
    }
}

fn tr(key: &str) -> String {
    t!(key).to_string()
}

fn card((icon, title_key, body_key): (LucideIcon, &str, &str), theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(div().text_color(theme.accent_brand).child(Icon::new(icon).size(theme.font_size_lg)))
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr(title_key)))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).line_height(relative(theme.line_height_body)).child(tr(body_key)))
}

fn header(theme: &ThemeTokens) -> Div {
    let text = div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).child(tr(TITLE_KEY)))
        .child(div().text_size(theme.font_size_sm).text_color(theme.text_secondary).line_height(relative(theme.line_height_body)).child(tr(SUBTITLE_KEY)));
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_lg)
        .child(Illustration::guide(GuidePose::Cheer).width(theme.onboarding_art_size))
        .child(div().flex_1().child(text))
}

impl WelcomeOverlay {
    fn actions(&self) -> Div {
        let theme = &self.theme;
        let (start, skip) = (self.on_start.clone(), self.on_skip.clone());
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.spacing_md)
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(REPLAY_HINT_KEY)))
            .child(
                div()
                    .flex()
                    .gap(theme.control_gap)
                    .child(Button::new(SKIP_ID).ghost().label(tr(SKIP_KEY)).on_click(move |_, w, cx| skip(w, cx)))
                    .child(Button::new(START_ID).primary().icon(LucideIcon::ArrowRight).label(tr(START_KEY)).on_click(move |_, w, cx| start(w, cx))),
            )
    }
}

impl RenderOnce for WelcomeOverlay {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme.clone();
        let dialog = div()
            .id(DIALOG_ID)
            .w(theme.onboarding_width)
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .rounded(theme.radius_xl)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .elevation_shadow(&theme)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(header(&theme))
            .child(div().flex().gap(theme.stack_gap).children(CARDS.map(|spec| card(spec, &theme))))
            .child(self.actions());
        let skip = self.on_skip.clone();
        div()
            .id(BACKDROP_ID)
            .absolute()
            .top(Pixels::ZERO)
            .left(Pixels::ZERO)
            .size_full()
            .occlude()
            .bg(theme.bg_scrim)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| skip(window, cx))
            .child(dialog)
    }
}

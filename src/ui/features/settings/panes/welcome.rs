//! General pane, welcome section: replay the first-run welcome that opens the Agent Forge.

use super::super::general_props::GeneralPaneProps;
use super::common::{body_text, card, muted_note, section, section_label, spaced_row};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::{Div, ParentElement, Styled};
use rust_i18n::t;

const REPLAY_ID: &str = "settings-replay-welcome";

/// "Welcome tour": a line on what it is and a button showing it again.
pub(super) fn render_welcome_section(theme: &ThemeTokens, props: &mut GeneralPaneProps) -> Div {
    let mut button = Button::new(REPLAY_ID).outline().icon(LucideIcon::Sparkles).label(t!("onboarding.replay_button").to_string());
    if let Some(replay) = props.on_replay_welcome.take() {
        button = button.on_click(move |_, window, cx| replay(window, cx));
    }
    section(
        section_label(theme, t!("onboarding.settings_title").to_string()),
        card(theme)
            .gap(theme.spacing_sm)
            .child(spaced_row().child(body_text(theme, t!("onboarding.settings_label").to_string())).child(button))
            .child(muted_note(theme, t!("onboarding.settings_note").to_string())),
    )
}

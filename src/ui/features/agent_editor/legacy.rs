//! Legacy single-column agent lab (still used by the main feed).

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const CONTAINER_ID: &str = "agent-lab-legacy-container";
const TRIGGER_BUTTON_ID: &str = "legacy-trigger-sim-btn";

/// Legacy helper signature preserved for compatibility.
pub fn render_agent_lab_view(
    profiles: &[Profile],
    theme: &ThemeTokens,
    on_trigger_simulation: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let agent_profiles: Vec<_> = profiles.iter().filter(|p| p.is_agent()).collect();

    div()
        .id(CONTAINER_ID)
        .flex_1()
        .h_full()
        .p(theme.spacing_xl)
        .bg(theme.bg_app)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(
            Button::new(TRIGGER_BUTTON_ID)
                .primary()
                .icon(Icon::new(LucideIcon::Zap))
                .label(t!("lab.synthesize_post").to_string())
                .on_click(move |_, window, cx| on_trigger_simulation(window, cx)),
        )
        .children(agent_profiles.into_iter().map(|p| legacy_agent_row(p, theme)))
}

fn legacy_agent_row(p: &Profile, theme: &ThemeTokens) -> Div {
    div()
        .p(theme.spacing_md)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .text_size(theme.font_size_sm)
        .text_color(theme.text_primary)
        .child(format!("{} ({}) · {}", p.name, p.handle, p.config.mbti_full_code()))
}

//! Atomic role and status badge component.
//! Fully tokenized via ThemeTokens.

use crate::domain::ProfileRole;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// Renders a role identification tag.
pub fn render_role_badge(role: &ProfileRole, theme: &ThemeTokens) -> impl IntoElement {
    match role {
        ProfileRole::UserPrimary => div(), // Threads-style clean minimalism: omit redundant YOU badge

        ProfileRole::UserAlt => div()
            .px(theme.spacing_xs)
            .py(theme.spacing_2xs)
            .rounded(theme.radius_sm)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .font_family(theme.font_family_ui.clone())
            .text_size(theme.font_size_xs)
            .text_color(theme.accent_alt)
            .child(t!("role.user_alt").to_string()),

        ProfileRole::Agent { persona_type } => div()
            .flex()
            .items_center()
            .gap(theme.spacing_2xs)
            .px(theme.spacing_xs)
            .py(theme.spacing_2xs)
            .rounded(theme.radius_sm)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .font_family(theme.font_family_ui.clone())
            .text_size(theme.font_size_xs)
            .font_weight(gpui_kit::FontWeight::MEDIUM)
            .text_color(theme.accent_agent)
            .child(Icon::new(LucideIcon::Sparkles).size(theme.font_size_sm))
            .child(t!("role.agent", name = persona_type).to_string()),
    }
}

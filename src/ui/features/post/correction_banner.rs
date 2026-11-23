//! N14: a retracted or corrected post wears a banner under its author: which it is, and the
//! author's reason. A locked post never wears one (it cannot be marked; a forged mark is not
//! shown either).

use crate::domain::post::correction::{Correction, CorrectionKind};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The mark `post` shows (`None`: unmarked, or locked).
pub(crate) fn shown_correction(post: &Post) -> Option<&Correction> {
    post.extras.correction.as_deref().filter(|_| !post.is_locked())
}

/// The banner's text: "Retracted: <reason>".
pub(crate) fn correction_text(correction: &Correction) -> String {
    let key = format!("posts.correction.banner_{}", correction.kind.key());
    t!(key.as_str(), reason = correction.reason.clone()).to_string()
}

/// The banner; `None` on an unmarked post.
pub(super) fn render_correction_banner(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let correction = shown_correction(post)?;
    let (icon, colour) = match correction.kind {
        CorrectionKind::Retracted => (LucideIcon::CircleX, theme.accent_danger),
        CorrectionKind::Corrected => (LucideIcon::Pencil, theme.accent_dialectic),
    };
    let text = div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(correction_text(correction));
    let banner = div()
        .w_full()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border_l(theme.border_width_threadline)
        .border_color(colour)
        .child(Icon::new(icon).small().text_color(colour))
        .child(text);
    Some(banner)
}

//! A feeling in the feed (M3): one line, set apart from thoughts by a callout tint and a
//! heart-pulse mark: the emojis, the intensity as five pips, the note, and who felt it when.

use super::consts::{META_SEPARATOR, POST_ITEM_ID_PREFIX, TIME_FORMAT};
use super::controls::PostCardHandlers;
use crate::domain::post::{Intensity, PostKind};
use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled};

/// The five intensity pips, `intensity` of them filled.
fn pips(intensity: Intensity, theme: &ThemeTokens) -> Div {
    let pip = |filled: bool| div().size(theme.dot_size_sm).rounded(theme.radius_full).bg(if filled { theme.accent_brand } else { theme.border_subtle });
    div().flex().items_center().gap(theme.spacing_2xs).children(Intensity::ALL.map(|step| pip(step <= intensity)))
}

/// The emojis, pips and note of a feeling (`None` for any other kind).
pub fn feeling_line(kind: &PostKind, theme: &ThemeTokens) -> Option<Div> {
    let PostKind::Feeling { emojis, intensity, note } = kind else { return None };
    let glyphs = div().flex_none().text_size(theme.font_size_lg).child(emojis.concat());
    let note = (!note.is_empty()).then(|| div().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(note.clone()));
    Some(div().flex_1().min_w_0().flex().items_center().gap(theme.control_gap).child(glyphs).child(pips(*intensity, theme)).children(note))
}

/// The one-line row of the feeling `post` by `author`.
pub fn render_feeling_row(post: &Post, author: &Profile, theme: &ThemeTokens, is_focused: bool, handlers: &PostCardHandlers) -> Option<impl IntoElement> {
    let line = feeling_line(&post.kind, theme)?;
    let on_select = handlers.on_select.clone();
    let when = post.created_at.strftime(TIME_FORMAT).to_string();
    let who = div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(format!("{}{META_SEPARATOR}{when}", author.name));
    let mark = Icon::new(LucideIcon::HeartPulse).xsmall().text_color(theme.accent_brand);
    let background = if is_focused { theme.hover_bg } else { theme.bg_callout };
    Some(
        div()
            .id(format!("{POST_ITEM_ID_PREFIX}{}", post.id))
            .w_full()
            .px(theme.panel_padding)
            .py(theme.spacing_xs)
            .bg(background)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .on_mouse_down(MouseButton::Left, move |_, window, cx| on_select(window, cx))
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(mark)
            .child(line)
            .child(who),
    )
}

#[cfg(test)]
mod tests {
    use super::feeling_line;
    use crate::domain::post::{Intensity, PostKind};
    use crate::theme::ThemeTokens;

    #[test]
    fn only_feelings_get_the_one_line_row() {
        let theme = ThemeTokens::default();
        let calm = PostKind::feeling(&["😌".to_string()], Intensity::new(2), "after the walk").expect("valid");
        assert!(feeling_line(&calm, &theme).is_some());
        assert!(feeling_line(&PostKind::Standard, &theme).is_none());
        assert!(feeling_line(&PostKind::Question, &theme).is_none());
    }
}

//! N12: an ink drawing on its card, painted from its strokes in the theme's colours, with a
//! button that opens the saved SVG. A locked post shows nothing of it: only the placeholder.

use crate::domain::post::ink::InkDrawing;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::components::ink_figure::ink_figure;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The drawing `post` shows (`None`: none, or the post is locked).
pub(crate) fn shown_ink(post: &Post) -> Option<&InkDrawing> {
    post.extras.ink.as_deref().filter(|_| !post.is_locked())
}

/// The card's drawing; `None` without one (or on a locked post).
pub(super) fn render_ink_card(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let drawing = shown_ink(post)?;
    let figure = div().w_full().h(theme.ink_card_height).rounded(theme.radius_md).bg(theme.bg_callout).child(ink_figure(drawing.clone(), theme));
    let open = drawing.svg_path.clone().map(|path| {
        Button::new(SharedString::from(format!("ink-open-{}", post.id)))
            .ghost()
            .xsmall()
            .icon(Icon::new(LucideIcon::ExternalLink))
            .label(t!("posts.ink.open_svg").to_string())
            .on_click(move |_, _, _| {
                let _ = crate::media::files::open_file(std::path::Path::new(&path));
            })
    });
    Some(div().w_full().flex().flex_col().gap(theme.spacing_3xs).child(figure).children(open))
}

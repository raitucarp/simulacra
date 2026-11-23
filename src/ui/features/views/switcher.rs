//! The view switcher: one small icon button per view the scope offers, the active one in the
//! accent. The thread inspector's header and the profile page both show it.

use super::registry::{view_icon, view_label};
use crate::domain::views::{PostView, ViewScope};
use crate::theme::ThemeTokens;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, App, Div, ParentElement, SharedString, Styled, Window};
use std::rc::Rc;

/// Picks a view.
pub type PickView = Rc<dyn Fn(PostView, &mut Window, &mut App) + 'static>;

/// Element id of `view`'s button in a switcher whose ids start with `prefix`.
pub fn switcher_button_id(prefix: &str, view: PostView) -> SharedString {
    SharedString::from(format!("{prefix}-{}", view.key()))
}

/// The buttons of `scope`'s views, `active` selected.
pub fn render_view_switcher(scope: ViewScope, active: PostView, id_prefix: &str, on_pick: &PickView, theme: &ThemeTokens) -> Div {
    let buttons = PostView::in_scope(scope).into_iter().map(|view| {
        let on_pick = on_pick.clone();
        let color = if view == active { theme.accent_brand } else { theme.text_muted };
        Button::new(switcher_button_id(id_prefix, view))
            .ghost()
            .xsmall()
            .selected(view == active)
            .icon(Icon::new(view_icon(view)).text_color(color))
            .tooltip(view_label(view))
            .on_click(move |_, window, cx| on_pick(view, window, cx))
    });
    div().flex_none().flex().items_center().gap(theme.spacing_3xs).children(buttons)
}

//! Modal header: cancel action, mode title over who will see the post, and the "more options"
//! menu.

use super::super::consts::{MODAL_CANCEL_BTN_ID, MODAL_MORE_BTN_ID};
use super::{ComposerModalView, ModalActionCallback};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    pub(super) fn header(&self) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .h(theme.feed_header_height)
            .px(theme.panel_padding)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .child(self.cancel_button())
            .child(self.title_block())
            .child(self.more_options_button())
    }

    /// The mode title over the audience line ("Visible to members of this network"), which the
    /// crowded footer used to cut to "…".
    fn title_block(&self) -> Div {
        let theme = &self.theme;
        let title = div()
            .font_family(theme.font_family_display.clone())
            .font_weight(FontWeight::BOLD)
            .text_size(theme.font_size_base)
            .text_color(theme.text_primary)
            .child(self.title_text());
        let audience = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("composer.post_options").to_string());
        div().flex().flex_col().items_center().child(title).child(audience)
    }

    fn cancel_button(&self) -> Stateful<Div> {
        let theme = &self.theme;
        let on_close_cancel = self.callbacks.on_close.clone();
        div()
            .id(MODAL_CANCEL_BTN_ID)
            .cursor_pointer()
            .text_size(theme.font_size_sm)
            .text_color(theme.text_secondary)
            .hover(move |s| s.text_color(theme.text_primary))
            .on_click(move |_, window, cx| {
                (on_close_cancel)(window, cx);
            })
            .child(t!("composer.cancel").to_string())
    }

    /// "…" menu: add a post to the thread; attach a link (ticked while its field is open),
    /// pick an image, insert a hashtag.
    fn more_options_button(&self) -> impl IntoElement {
        let (callbacks, link_open) = (self.callbacks.clone(), self.is_link_input_open);
        Button::new(MODAL_MORE_BTN_ID)
            .ghost()
            .xsmall()
            .icon(Icon::new(LucideIcon::MoreHorizontal))
            .tooltip(t!("composer.more_options").to_string())
            .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, window, cx| render_menu(menu, &more_options_spec(&callbacks, link_open), window, cx))
    }
}

/// The composer's "More options" menu.
pub(super) fn more_options_spec(callbacks: &super::ModalCallbacks, link_open: bool) -> MenuSpec {
    let thread = MenuSection::new().entry(menu_item("composer.add_to_thread", LucideIcon::Plus, &callbacks.on_add_draft));
    let attach = MenuSection::new()
        .entry(menu_item("composer.attach_link", LucideIcon::Link2, &callbacks.on_toggle_link).checked(link_open))
        .entry(menu_item("composer.pick_image", LucideIcon::Image, &callbacks.on_pick_image))
        .entry(menu_item("composer.insert_tag", LucideIcon::Hash, &callbacks.on_insert_tag));
    MenuSpec::new().section(thread).section(attach)
}

fn menu_item(key: &'static str, icon: LucideIcon, action: &ModalActionCallback) -> MenuItem {
    let action = action.clone();
    MenuItem::new(key, MenuText::key(key)).icon(icon).on_click(move |window, cx| action(window, cx))
}

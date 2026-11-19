//! The vault screen: a facet rail (a drawer on compact windows) beside the current screen.

mod browse;
mod common;
mod cover_view;
mod editor;
mod empty;
mod entry;
mod facet_row;
mod facets;
mod filters;
mod reader;
mod shelf;
mod subject_index;
mod table;
mod toolbar;

pub use empty::{EMPTY_VAULT_ART, NO_MATCHES_ART};
pub use subject_index::SUBJECT_INDEX_ART;

use super::panel::VaultPanel;
use super::screen::Screen;
use crate::ui::responsive::LayoutClass;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const VAULT_ROOT_ID: &str = "vault-root";
const DRAWER_SCRIM_ID: &str = "vault-drawer-scrim";

impl Render for VaultPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let compact = LayoutClass::of_window(window, &self.theme).is_compact();
        let browsing = self.nav.current == Screen::Browse && !self.data.documents.is_empty();
        let screen = match self.nav.current.clone() {
            Screen::Browse => self.render_browse(compact, cx).into_any_element(),
            Screen::Entry(id) => self.render_entry(&id, compact, cx),
            Screen::Reader(id) => self.render_reader(&id, cx),
            Screen::SubjectIndex => self.render_subject_index(cx),
            Screen::Editor(id) => self.render_editor(id.is_some(), compact, cx),
        };
        div()
            .id(VAULT_ROOT_ID)
            .size_full()
            .relative()
            .flex()
            .bg(self.theme.bg_app)
            .when(browsing && !compact, |root| root.child(self.render_facet_rail(cx)))
            .child(div().flex_1().min_w_0().h_full().child(screen))
            .when(browsing && compact && self.drawer_open, |root| root.child(self.render_drawer(cx)))
    }
}

impl VaultPanel {
    /// The facets as a drawer over a scrim (compact windows).
    fn render_drawer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .absolute()
            .inset_0()
            .flex()
            .child(
                div()
                    .w(theme.vault_facet_drawer_width)
                    .h_full()
                    .bg(theme.bg_sidebar)
                    .border_r(theme.border_width_hairline)
                    .border_color(theme.border)
                    .child(self.render_facets(cx)),
            )
            .child(
                div()
                    .id(DRAWER_SCRIM_ID)
                    .flex_1()
                    .h_full()
                    .bg(theme.bg_scrim)
                    .on_click(cx.listener(|this, _, _, cx| this.set_drawer(false, cx))),
            )
    }

    /// The facets as a column beside the results.
    fn render_facet_rail(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .w(theme.vault_facet_rail_width)
            .flex_shrink_0()
            .h_full()
            .bg(theme.bg_sidebar)
            .border_r(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(self.render_facets(cx))
    }
}

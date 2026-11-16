//! Root render: title bar, the three resizable columns and the floating overlays.

mod center;
mod columns;
mod overlays;
mod settings;
mod sidebars;
mod workspace;

use super::binder::Binder;
use super::view::AppView;
use gpui_kit::*;

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_composer_view(cx);
        let bind = Binder::new(cx);
        let columns = self.render_columns(&bind, cx);

        div()
            .w_full()
            .h_full()
            .bg(self.theme.bg_app)
            .relative()
            .flex()
            .flex_col()
            .child(self.title_bar(&bind))
            // Fully Resizable Multi-Column Layout Container
            .child(div().w_full().flex_1().overflow_hidden().child(columns))
            .children(self.language_popover(&bind))
            .children(self.settings_modal(&bind))
            .children(self.workspace_popover(&bind))
            .children(self.composer_modal_view(&bind, cx))
    }
}

impl AppView {
    /// Pushes the active profile and theme into the isolated inline composer view.
    fn sync_composer_view(&self, cx: &mut Context<Self>) {
        let current_profile = self.state.current_profile();
        let current_theme = &self.theme;
        self.feed.composer_view.update(cx, |composer, cx| {
            composer.set_profile(current_profile, cx);
            composer.set_theme(current_theme, cx);
        });
    }
}

//! Root render: title bar, the three resizable columns and the floating overlays.

mod center;
mod columns;
mod drawer;
mod overlays;
mod settings;
mod sidebars;
mod workspace;

use super::binder::Binder;
use super::view::AppView;
use crate::ui::render_stats;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

/// Name of the root view in the dev render counter.
const ROOT_STATS_NAME: &str = "AppView";

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        render_stats::record(ROOT_STATS_NAME);
        self.sync_composer_view(cx);
        self.sync_stream(cx);
        self.sync_mind_graph(cx);
        self.sync_forge_theme(cx);
        let bind = Binder::new(cx);
        let class = LayoutClass::of_window(window, &self.theme);
        let columns = self.render_columns(class);

        div()
            .w_full()
            .h_full()
            .bg(self.theme.bg_app)
            .relative()
            .flex()
            .flex_col()
            .child(self.title_bar(&bind))
            // Fully Resizable Multi-Column Layout Container
            .child(div().w_full().flex_1().min_h_0().overflow_hidden().child(columns))
            .children(self.right_drawer(&bind, class))
            .children(self.language_popover(&bind))
            .children(self.settings_modal(&bind, cx))
            .children(self.workspace_popover(&bind))
            .children(self.composer_modal_view(cx))
            .children(self.welcome_overlay(&bind))
    }
}

impl AppView {
    /// Pushes the active profile and theme into the isolated composer entities.
    fn sync_composer_view(&self, cx: &mut Context<Self>) {
        let current_profile = self.state.current_profile();
        let current_theme = &self.theme;
        self.feed.composer_view.update(cx, |composer, cx| {
            composer.set_profile(current_profile, cx);
            composer.set_theme(current_theme, cx);
        });
        self.composer_modal.update(cx, |panel, cx| {
            panel.set_profile(current_profile, cx);
            panel.set_theme(current_theme, cx);
        });
    }
}

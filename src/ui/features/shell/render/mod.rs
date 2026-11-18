//! Root render: title bar, the three resizable columns and the floating overlays.

mod center;
mod columns;
mod drawer;
mod navigation;
mod image_ideas;
mod import;
mod knowledge; // sb F2: the Library / Saved switch
mod media;
mod overlays;
mod post_actions;
mod semantic;
mod settings;
mod sidebars;
mod workspace;

use super::binder::Binder;
use super::view::AppView;
use crate::ui::keymap::SHELL_KEY_CONTEXT;
use crate::ui::render_stats;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

/// Name of the root view in the dev render counter.
const ROOT_STATS_NAME: &str = "AppView";

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        render_stats::record(ROOT_STATS_NAME);
        self.sync_views(window, cx);
        let bind = Binder::new(cx);
        let class = LayoutClass::of_window(window, &self.theme);
        let columns = self.journal_focus_view().unwrap_or_else(|| self.render_columns(class).into_any_element());

        let root = div()
            .w_full()
            .h_full()
            .bg(self.theme.bg_app)
            .relative()
            // Global shortcuts (the central keymap's `RunCommand`) reach the shell from anywhere
            .key_context(SHELL_KEY_CONTEXT)
            .track_focus(&self.palette.shell_focus)
            .on_action(cx.listener(Self::on_run_command))
            .flex()
            .flex_col();
        self.with_import_listeners(root, cx)
            .child(self.title_bar(&bind))
            // Fully Resizable Multi-Column Layout Container
            .child(div().w_full().flex_1().min_h_0().overflow_hidden().child(columns))
            .children(self.right_drawer(&bind, class))
            .children(self.language_popover(&bind))
            .children(self.settings_modal(&bind, cx))
            .children(self.workspace_popover(&bind))
            .children(self.composer_modal_view(cx))
            .children(self.welcome_overlay(&bind))
            .children(self.indexing_indicator(&bind))
            .children(self.similar_sheet(&bind))
            .children(self.post_actions_sheet(&bind)).children(self.atomic_sheet_overlay(&bind))
            .children(self.image_ideas_sheet(&bind))
            .children(self.palette.palette.clone())
            .children(self.shortcut_sheet(&bind))
            .children(self.import_sheet(&bind, cx))
            .children(self.capture_box(&bind))
            .children(self.learn_sheet(cx))
            .children(self.drop_hint(cx))
            .children(self.lightbox(&bind)).children(self.data_overlay(&bind)) // sb F2
    }
}

impl AppView {
    /// Render hook: focus, then every child view and background index brought up to date.
    fn sync_views(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.claim_idle_focus(window, cx);
        self.sync_composer_view(cx);
        self.sync_stream(cx);
        self.sync_mind_graph(cx);
        self.sync_vault(cx);
        self.sync_semantic_index(window, cx);
        self.sync_media(window, cx);
        self.sync_review(window, cx);
        self.sync_insights(cx);
        self.sync_journal(window, cx);
        self.sync_thinking(cx);
        self.sync_forge_theme(cx);
    }

    /// The learn sheet, while open (sb C1b).
    fn learn_sheet(&self, cx: &App) -> Option<Entity<crate::ui::features::review::LearnSheet>> {
        self.review.learn.read(cx).is_open().then(|| self.review.learn.clone())
    }

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
        let media_dir = self.state.media_dir();
        self.composer_modal.update(cx, |panel, _| panel.set_media_dir(media_dir));
    }
}

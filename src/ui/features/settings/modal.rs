//! The settings window element: the searchable category rail beside the active category
//! (or the search results), framed to fit inside the app window.

use super::callbacks::SettingsActions;
use super::catalog::{search, search_entries, SearchHit};
use super::consts::BODY_ID;
use super::credentials::CredentialsPaneProps;
use super::layout::{render_category, render_footer, render_frame, render_header, render_rail, render_search, search_box, EmbeddedPanes, HeaderSubject, RailView};
use super::panes::AppearancePanel;
use super::props::{AiPaneProps, SettingsData, SettingsInputs};
use super::row::RowContext;
use super::tabs::SettingsTab;
use crate::theme::ThemeTokens;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

/// The settings window.
#[derive(IntoElement)]
pub struct SettingsModal {
    theme: ThemeTokens,
    active_tab: SettingsTab,
    data: SettingsData,
    inputs: SettingsInputs,
    actions: SettingsActions,
    panes: EmbeddedPanes,
}

impl SettingsModal {
    pub fn new(
        theme: ThemeTokens,
        active_tab: SettingsTab,
        data: SettingsData,
        inputs: SettingsInputs,
        actions: SettingsActions,
        appearance: Entity<AppearancePanel>,
    ) -> Self {
        let panes = EmbeddedPanes { ai: None, credentials: None, appearance: Some(appearance), scheduler: data.scheduler.clone() };
        Self { theme, active_tab, data, inputs, actions, panes }
    }

    /// Supplies the AI provider pane.
    pub fn ai(mut self, ai: AiPaneProps) -> Self {
        self.panes.ai = Some(ai);
        self
    }

    /// Supplies the credentials list, editor and callbacks.
    pub fn credentials(mut self, credentials: CredentialsPaneProps) -> Self {
        self.panes.credentials = Some(credentials);
        self
    }

    /// The search hits of the current query (none when not searching).
    fn hits(&self) -> Vec<SearchHit> {
        match self.data.query.trim() {
            "" => Vec::new(),
            query => search(query, &search_entries()),
        }
    }
}

impl RenderOnce for SettingsModal {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let hits = self.hits();
        let searching = !self.data.query.trim().is_empty();
        let Self { theme, active_tab, data, inputs, actions, panes } = self;
        let theme = &theme;
        let ctx = RowContext { theme, data: &data, inputs: &inputs, actions: &actions };
        let subject = if searching { HeaderSubject::Search(hits.len()) } else { HeaderSubject::Tab(active_tab) };
        let body = if searching { render_search(&ctx, &hits) } else { render_category(&ctx, active_tab, panes) };
        let show_save = !searching && active_tab == SettingsTab::AiIntelligence;
        let compact = LayoutClass::of_window(window, theme).is_compact();
        let top_search = compact.then(|| div().flex_none().px(theme.panel_padding).pt(theme.panel_padding_compact).child(search_box(theme, &inputs)));
        let content = div()
            .size_full()
            .min_w_0()
            .overflow_hidden()
            .flex()
            .flex_col()
            .children(top_search)
            .child(render_header(theme, subject, actions.on_close.clone()))
            .child(
                div()
                    .id(SharedString::from(format!("{BODY_ID}-{}", if searching { "search" } else { active_tab.id() })))
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .overflow_y_scrollbar()
                    .p(theme.panel_padding)
                    .child(body),
            )
            .child(render_footer(theme, &actions, data.ai_dirty, show_save));
        let rail = RailView { active: active_tab, data: &data, inputs: &inputs, hits: &hits, on_select: actions.on_select.clone(), compact };
        render_frame(theme, window, (actions.on_close.clone(), compact), render_rail(theme, rail), content)
    }
}

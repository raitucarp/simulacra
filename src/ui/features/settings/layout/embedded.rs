//! The larger panes hosted as rows of a category: each is handed out once.

use super::super::catalog::EmbeddedPane;
use super::super::credentials::{render_credentials_section, CredentialsPaneProps};
use super::super::panes::{render_ai_pane, render_scheduler_status, AppearancePanel};
use super::super::props::{AiPaneProps, SchedulerStatus};
use crate::theme::ThemeTokens;
use gpui_kit::{AnyElement, Entity, IntoElement};

/// The hosted panes of one render.
pub(in crate::ui::features::settings) struct EmbeddedPanes {
    pub ai: Option<AiPaneProps>,
    pub credentials: Option<CredentialsPaneProps>,
    pub appearance: Option<Entity<AppearancePanel>>,
    pub scheduler: SchedulerStatus,
    pub symbols: Option<super::super::panes::SymbolSetsProps>,
    pub licences: Option<Vec<super::super::panes::LicenceView>>,
}

impl EmbeddedPanes {
    /// Whether `pane` brings its own section headings (it is not wrapped in a card).
    pub const fn is_block(pane: EmbeddedPane) -> bool {
        !matches!(pane, EmbeddedPane::SchedulerStatus)
    }

    /// The element of `pane` (the first time only).
    pub fn take(&mut self, theme: &ThemeTokens, pane: EmbeddedPane) -> Option<AnyElement> {
        match pane {
            EmbeddedPane::AiProvider => self.ai.take().map(|ai| render_ai_pane(theme, ai).into_any_element()),
            EmbeddedPane::Credentials => self.credentials.take().map(|props| render_credentials_section(theme, props).into_any_element()),
            EmbeddedPane::Themes => self.appearance.take().map(IntoElement::into_any_element),
            EmbeddedPane::SymbolSets => self.symbols.take().map(|props| super::super::panes::render_symbol_sets(theme, props).into_any_element()),
            EmbeddedPane::Licences => self.licences.take().map(|licences| super::super::panes::render_licences(theme, &licences).into_any_element()),
            EmbeddedPane::SchedulerStatus => Some(render_scheduler_status(theme, &self.scheduler).into_any_element()),
        }
    }
}

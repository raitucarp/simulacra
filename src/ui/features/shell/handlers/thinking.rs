//! The thinking tools' entry points in the shell (sb E1): palette commands, the post actions
//! that open a tool, the requests of the Calibration and Tensions views, and the render hook
//! that keeps the Argument view, the composer hint and the Insights views up to date.

use super::super::view::AppView;
use crate::domain::post_actions::ActionKind;
use crate::domain::PostId;
use crate::state::NavigationTab;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::feed::inspector_outline::InspectorMode;
use crate::ui::features::thinking::insights::ThinkingRequest;
use gpui_kit::*;

impl AppView {
    /// Palette commands: the two Insights views, due predictions, the argument map.
    pub(in crate::ui::features::shell) fn run_thinking_command(&mut self, id: CommandId, window: &mut Window, cx: &mut Context<Self>) {
        match id {
            CommandId::ResolveDuePredictions => {
                self.thinking.due_only = true;
                self.open_insights(CommandId::InsightsCalibration, window, cx);
            }
            CommandId::ShowArgumentMap => {
                self.select_nav(NavigationTab::Timeline, window, cx);
                self.set_inspector_mode(InspectorMode::Argument, window, cx);
            }
            _ => self.open_insights(id, window, cx),
        }
    }

    /// "…" › Actions › Map the argument / Find tensions / Check atomicity.
    pub(in crate::ui::features::shell) fn open_thinking_tool(&mut self, post_id: PostId, kind: ActionKind, window: &mut Window, cx: &mut Context<Self>) {
        match kind {
            ActionKind::MapArgument => self.map_argument_of(post_id, window, cx),
            ActionKind::FindTensions => self.find_tensions_with(post_id, window, cx),
            ActionKind::CheckAtomicity => self.check_atomicity_of(post_id, window, cx),
            _ => {}
        }
    }

    /// A request from the Calibration or Tensions view.
    pub(in crate::ui::features::shell) fn on_thinking_request(&mut self, request: ThinkingRequest, window: &mut Window, cx: &mut Context<Self>) {
        match request {
            ThinkingRequest::OpenPost(id) => self.reveal_post(id, window, cx),
            ThinkingRequest::OpenDocument(id) => self.open_vault_document(&id, window, cx),
            ThinkingRequest::ResolvePrediction(id, outcome) => self.resolve_prediction(id, outcome, cx),
            ThinkingRequest::ToggleDueOnly => {
                self.thinking.due_only = !self.thinking.due_only;
                cx.notify();
            }
            ThinkingRequest::ScanTensions => self.rescan_tensions(cx),
            ThinkingRequest::ClearFocus => {
                self.thinking.scan.focus = None;
                self.rescan_tensions(cx);
            }
            ThinkingRequest::Reconcile(pair) => self.reconcile_tension(&pair, window, cx),
            ThinkingRequest::MarkResolved(pair) => self.set_tension_status(&pair, crate::domain::thinking::tensions::TensionStatus::Resolved, cx),
            ThinkingRequest::Dismiss(pair) => self.set_tension_status(&pair, crate::domain::thinking::tensions::TensionStatus::Dismissed, cx),
        }
    }

    /// Render hook: the Argument view, the composer's hint settings and, while the Insights
    /// page is open, the Calibration and Tensions data.
    pub(in crate::ui::features::shell) fn sync_thinking(&mut self, cx: &mut Context<Self>) {
        let theme = self.theme.clone();
        self.thinking.argument.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        self.sync_argument_panel(cx);
        self.sync_atomic_hints(cx);
        if self.state.active_tab == NavigationTab::Insights {
            self.sync_thinking_insights(cx);
        }
    }
}

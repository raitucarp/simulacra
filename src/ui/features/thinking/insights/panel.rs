//! The Insights page's hook for the thinking views: the shell hands it a fresh snapshot.

use super::ThinkingInsights;
use crate::ui::features::insights::InsightsPanel;
use gpui_kit::Context;

impl InsightsPanel {
    /// Swaps in the Calibration and Tensions data (repaints only when it changed).
    pub fn set_thinking(&mut self, thinking: ThinkingInsights, cx: &mut Context<Self>) {
        if self.thinking != thinking {
            self.thinking = thinking;
            cx.notify();
        }
    }
}

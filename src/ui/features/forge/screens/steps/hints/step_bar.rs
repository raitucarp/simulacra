//! The step's suggestion bar: "Suggest with AI" for every suggestible field of the page, "Fill
//! the rest" with the catalog's defaults, and what the last of them did.

use super::super::super::common::{card, message};
use crate::state::forge::suggest::ai::SuggestScope;
use crate::state::forge::ForgeStep;
use crate::ui::features::forge::text::{tr, tr_count, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView, SuggestStatus};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const TITLE_KEY: &str = "suggest.ui.bar_title";
const BODY_KEY: &str = "suggest.ui.bar_body";
const SUGGEST_KEY: &str = "suggest.ui.suggest_step";
const STOP_KEY: &str = "suggest.ui.stop";
const FILL_KEY: &str = "suggest.ui.fill_rest";
const FILL_TOOLTIP_KEY: &str = "suggest.ui.fill_tooltip";
const THINKING_KEY: &str = "suggest.ui.thinking";
const READY_KEY: &str = "suggest.ui.ai_ready";
const DISCARDED_KEY: &str = "suggest.ui.ai_discarded";
const FILLED_KEY: &str = "suggest.ui.fill_done";
const FAILED_KEY: &str = "suggest.ui.ai_failed";
const USE_ALL_KEY: &str = "suggest.ui.use_all";
const DISMISS_KEY: &str = "suggest.ui.dismiss";
const ARG_ERROR: &str = "error";
const SUGGEST_ID: &str = "forge-suggest-step";
const STOP_ID: &str = "forge-suggest-stop";
const FILL_ID: &str = "forge-suggest-fill";
const USE_ALL_ID: &str = "forge-suggest-use-all";
const DISMISS_ID: &str = "forge-suggest-dismiss";

impl ForgeView {
    /// The status line under the bar's buttons.
    fn suggest_status_lines(&self) -> Vec<Div> {
        let theme = &self.theme;
        if self.suggest.run.is_some() {
            return vec![message(tr(THINKING_KEY), theme.accent_brand, theme)];
        }
        match &self.suggest.status {
            SuggestStatus::Idle => Vec::new(),
            SuggestStatus::Ready { count, discarded } => {
                let ready = message(tr_count(READY_KEY, *count, &[]), theme.accent_brand, theme);
                let skipped = (*discarded > 0).then(|| message(tr_count(DISCARDED_KEY, *discarded, &[]), theme.text_muted, theme));
                std::iter::once(ready).chain(skipped).collect()
            }
            SuggestStatus::Filled(count) => vec![message(tr_count(FILLED_KEY, *count, &[]), theme.accent_success, theme)],
            SuggestStatus::Note(key) => vec![message(tr(key), theme.text_muted, theme)],
            SuggestStatus::Failed(error) => vec![message(tr_with(FAILED_KEY, &[(ARG_ERROR, error.clone())]), theme.accent_danger, theme)],
        }
    }

    fn suggest_buttons(&self, step: ForgeStep, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let running = self.suggest.run.is_some();
        let ask = handle.act(move |this, _, cx| this.request_suggestions(SuggestScope::Step(step), cx));
        let stop = handle.act(|this, _, cx| this.cancel_suggestions(cx));
        let fill_rest = handle.act(|this, _, cx| this.fill_rest(cx));
        let suggest = Button::new(SUGGEST_ID).small().outline().icon(LucideIcon::Sparkles).label(tr(SUGGEST_KEY)).tooltip(tr(BODY_KEY)).loading(running);
        let suggest = suggest.on_click(move |_, w, cx| ask(w, cx));
        let stop = running.then(|| Button::new(STOP_ID).small().ghost().icon(LucideIcon::CircleStop).label(tr(STOP_KEY)).on_click(move |_, w, cx| stop(w, cx)));
        let fill = Button::new(FILL_ID).small().ghost().icon(LucideIcon::WandSparkles).label(tr(FILL_KEY)).tooltip(tr(FILL_TOOLTIP_KEY));
        div().flex().flex_wrap().items_center().gap(theme.spacing_xs).child(suggest).children(stop).child(fill.on_click(move |_, w, cx| fill_rest(w, cx)))
    }

    fn result_buttons(&self, handle: &ForgeHandle) -> Option<Div> {
        if self.suggest.results.is_empty() {
            return None;
        }
        let (use_all, dismiss) = (handle.act(|this, _, cx| this.accept_all_suggestions(cx)), handle.act(|this, _, cx| this.dismiss_suggestions(cx)));
        let use_all = Button::new(USE_ALL_ID).xsmall().primary().icon(LucideIcon::ListChecks).label(tr(USE_ALL_KEY)).on_click(move |_, w, cx| use_all(w, cx));
        let dismiss = Button::new(DISMISS_ID).xsmall().ghost().label(tr(DISMISS_KEY)).on_click(move |_, w, cx| dismiss(w, cx));
        Some(div().flex().items_center().gap(self.theme.spacing_xs).child(use_all).child(dismiss))
    }

    /// The bar of the current step (none on Review).
    pub(in super::super) fn suggest_bar(&self, handle: &ForgeHandle) -> Option<Div> {
        let step = self.draft.step();
        if step == ForgeStep::Review {
            return None;
        }
        let theme = &self.theme;
        let title = div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .text_size(theme.font_size_sm)
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.text_primary)
            .child(Icon::new(LucideIcon::Lightbulb).size(theme.font_size_sm).text_color(theme.accent_dialectic))
            .child(tr(TITLE_KEY));
        Some(
            card(theme)
                .gap(theme.spacing_sm)
                .child(title)
                .child(self.suggest_buttons(step, handle))
                .children(self.suggest_status_lines())
                .children(self.result_buttons(handle)),
        )
    }
}

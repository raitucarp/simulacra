//! Review: the trading card, every issue grouped by step with a "Fix" link, and "Bring to
//! life" (save, switch the agent on, optionally write an introduction post).

use super::super::super::common::{card, heading, muted};
use super::issue_text;
use crate::state::forge::{can_save, EngineSection, ForgeKind, Severity, ValidationIssue};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{tr, tr_count};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ToggleCard;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::component::Disableable;
use gpui_kit::*;

const ISSUES_KEY: &str = "forge.ui.review.issues_title";
const NO_ISSUES_KEY: &str = "forge.ui.review.no_issues";
const FIX_KEY: &str = "forge.ui.review.fix";
const LIFE_KEY: &str = "forge.ui.review.bring_to_life";
const INTRO_KEY: &str = "forge.ui.review.intro_post";
const INTRO_HINT_KEY: &str = "forge.ui.review.intro_hint";
const BLOCKED_KEY: &str = "forge.ui.save_blocked";
const FIX_ID_PREFIX: &str = "forge-fix";
const LIFE_ID: &str = "forge-bring-to-life";
const INTRO_ID: &str = "forge-intro-post";

fn issue_row(at: usize, issue: &ValidationIssue, theme: &ThemeTokens, handle: &ForgeHandle) -> Div {
    let (icon, color) = match issue.severity {
        Severity::Error => (LucideIcon::CircleAlert, theme.accent_danger),
        Severity::Warning => (LucideIcon::Info, theme.warning),
    };
    let (step, section) = (issue.step, EngineSection::of_field(issue.field));
    let fix = handle.act(move |this, window, cx| {
        if let Some(section) = section {
            this.engine.section = section;
        }
        this.go_to(step, window, cx);
    });
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(div().text_color(color).child(Icon::new(icon).size(theme.font_size_md)))
        .child(div().flex_1().text_size(theme.font_size_sm).child(issue_text(issue)))
        .child(Button::new((FIX_ID_PREFIX, at)).ghost().xsmall().label(tr(FIX_KEY)).on_click(move |_, w, cx| fix(w, cx)))
}

impl ForgeView {
    pub(in super::super) fn review_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .child(self.trading_card())
            .child(self.issue_list(handle))
            .child(self.bring_to_life_panel(handle))
            .into_any_element()
    }

    fn issue_list(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        if self.issues.is_empty() {
            return card(theme).child(heading(tr(ISSUES_KEY), theme)).child(muted(tr(NO_ISSUES_KEY), theme));
        }
        let groups: Vec<Div> = self
            .draft
            .steps()
            .iter()
            .filter_map(|&step| {
                let rows: Vec<Div> = self
                    .issues
                    .iter()
                    .enumerate()
                    .filter(|(_, issue)| issue.step == step)
                    .map(|(at, issue)| issue_row(at, issue, theme, handle))
                    .collect();
                (!rows.is_empty()).then(|| {
                    let label = div().text_size(theme.font_size_xs).font_weight(FontWeight::BOLD).child(tr(&step.label_key()));
                    div().flex().flex_col().gap(theme.spacing_xs).child(label).children(rows)
                })
            })
            .collect();
        card(theme).child(heading(tr(ISSUES_KEY), theme)).children(groups)
    }

    fn bring_to_life_panel(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let ready = can_save(&self.issues);
        let errors = self.issues.iter().filter(|issue| issue.severity == Severity::Error).count();
        let go = handle.act(ForgeView::bring_to_life);
        let mut button = Button::new(LIFE_ID).primary().large().icon(LucideIcon::Sparkles).label(tr(LIFE_KEY)).disabled(!ready);
        if !ready {
            button = button.tooltip(tr_count(BLOCKED_KEY, errors, &[]));
        }
        let intro = (self.draft.kind() == ForgeKind::Agent).then(|| {
            let toggle = handle.with(|this, on: bool, _, cx| {
                this.intro_post = on;
                cx.notify();
            });
            ToggleCard::custom(INTRO_ID, tr(INTRO_KEY), self.intro_post, theme, move |on, w, cx| toggle(on, w, cx)).hint(Some(tr(INTRO_HINT_KEY)))
        });
        card(theme).children(intro).child(button.on_click(move |_, w, cx| go(w, cx)))
    }
}

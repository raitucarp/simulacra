//! [`HourRangePicker`]: when an agent is online, as a 24-cell day strip (active hours lit,
//! wrapping past midnight like the scheduler's `HourWindow`) plus start / end steppers.
//! Clicking an hour moves the start there; Shift-clicking moves the end.

use super::ValueCallback;
use crate::agent::schedule::{HourWindow, HOURS_PER_DAY};
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::fields::MAX_HOUR;
use crate::state::forge::HourField;
use crate::theme::ThemeTokens;
use crate::ui::components::mini_step_button;
use crate::ui::features::forge::text::{field_label, tr};
use gpui_kit::*;
use std::rc::Rc;

const STRIP_ID: &str = "forge-hours-strip";
const STEP_ID_PREFIX: &str = "forge-hours-step";
const HINT_KEY: &str = "forge.ui.hours_hint";
const MINUS: &str = "-";
const PLUS: &str = "+";

/// Clock format of one hour (`07:00`).
fn clock(hour: u8) -> String {
    format!("{hour:02}:00")
}

/// Start / end of the active window of an agent.
#[derive(IntoElement)]
pub struct HourRangePicker {
    start: u8,
    end: u8,
    theme: ThemeTokens,
    on_change: ValueCallback<(HourField, u8)>,
}

impl HourRangePicker {
    /// The picker for `cfg`'s active hours; `on_change` gets `(field, hour)` per change.
    pub fn new(cfg: &AgentConfig, theme: &ThemeTokens, on_change: impl Fn((HourField, u8), &mut Window, &mut App) + 'static) -> Self {
        Self {
            start: cfg.active_hours_start,
            end: cfg.active_hours_end,
            theme: theme.clone(),
            on_change: Rc::new(on_change),
        }
    }

    fn cell(&self, hour: u8) -> Stateful<Div> {
        let theme = &self.theme;
        let lit = HourWindow::new(self.start, self.end).contains(hour);
        let on_change = self.on_change.clone();
        div()
            .id((STRIP_ID, hour as usize))
            .flex_1()
            .h(theme.forge_swatch_size)
            .rounded(theme.radius_xs)
            .bg(if lit { theme.accent_brand } else { theme.bg_elevated })
            .cursor_pointer()
            .on_click(move |event, window, cx| {
                let field = if event.modifiers().shift { HourField::ActiveHoursEnd } else { HourField::ActiveHoursStart };
                on_change((field, hour), window, cx);
            })
    }

    fn stepper(&self, field: HourField, value: u8) -> Div {
        let theme = &self.theme;
        let button = |label: &'static str, next: u8| {
            let on_change = self.on_change.clone();
            let id = SharedString::from(format!("{STEP_ID_PREFIX}-{}-{label}", field.path()));
            mini_step_button(id, label, theme, move |window, cx| on_change((field, next), window, cx))
        };
        let previous = if value == 0 { MAX_HOUR } else { value - 1 };
        let next = if value >= MAX_HOUR { 0 } else { value + 1 };
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(field_label(field.path())))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing_xs)
                    .child(button(MINUS, previous))
                    .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_sm).child(clock(value)))
                    .child(button(PLUS, next)),
            )
    }
}

impl RenderOnce for HourRangePicker {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = &self.theme;
        let cells: Vec<Stateful<Div>> = (0..HOURS_PER_DAY).map(|hour| self.cell(hour)).collect();
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_sm)
            .child(div().flex().gap(theme.spacing_3xs).children(cells))
            .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(HINT_KEY)))
            .child(self.stepper(HourField::ActiveHoursStart, self.start))
            .child(self.stepper(HourField::ActiveHoursEnd, self.end))
    }
}

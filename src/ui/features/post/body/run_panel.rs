//! The output panel under a code block (P6): while the block runs, "Running…" and Cancel;
//! afterwards its saved run: the runtime and its version, how it ended, how long it took and
//! when, then what it printed. Saved runs show even while code running is off.

use super::code_run::CancelRunAction;
use super::layout::Item;
use super::BodyLinks;
use crate::domain::code::{CodeRun, Runtime, RunEnd};
use crate::theme::ThemeTokens;
use crate::ui::features::post::PostLookup;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const PANEL_ID: &str = "code-block-output";
const CANCEL_ID: &str = "code-block-cancel";
const OUTPUT_ID: &str = "code-block-output-text";

/// What the panel of one block shows.
pub(super) struct RunPanel {
    pub saved: Option<CodeRun>,
    /// The block is running now: its post, its place and Cancel.
    pub running: Option<(String, usize, CancelRunAction)>,
}

/// The panel of the block at `index`, when it has run or runs now.
pub(super) fn run_panel<L: PostLookup>(items: &[Item<'_>], index: usize, lookup: &L, links: &BodyLinks) -> Option<RunPanel> {
    let block = items[..index].iter().filter(|item| matches!(item, Item::Block { .. })).count();
    let saved = lookup.find_post(&links.scope).and_then(|post| post.extras.code_runs.iter().find(|run| run.block == block).cloned());
    let running = links.code.as_ref().filter(|code| code.running.iter().any(|(post, at)| *post == links.scope && *at == block)).map(|code| (links.scope.clone(), block, code.on_cancel.clone()));
    (saved.is_some() || running.is_some()).then_some(RunPanel { saved, running })
}

/// How a saved run ended, in one line (`Python 3.12.2 · Finished with exit code 0 in 0.3 s`).
pub fn run_line(run: &CodeRun) -> String {
    let seconds = format!("{:.1}", run.duration_ms as f64 / 1000.0);
    let ended = match (run.end, run.exit_code) {
        (RunEnd::Exited, Some(code)) => t!("code.run.exited", code = code, seconds = seconds).to_string(),
        (RunEnd::Exited, None) => t!("code.run.ended", seconds = seconds).to_string(),
        (RunEnd::TimedOut, _) => t!("code.run.timed_out", seconds = seconds).to_string(),
        (RunEnd::Cancelled, _) => t!("code.run.cancelled").to_string(),
        (RunEnd::OutputCapped, _) => t!("code.run.capped").to_string(),
        (RunEnd::Failed, _) => t!("code.output.failed").to_string(),
    };
    let runtime: &str = Runtime::from_key(&run.runtime).map_or(&run.runtime, |runtime| runtime.label());
    let when = run.ran_at.map(crate::ui::features::data::format_time).unwrap_or_default();
    [format!("{runtime} {}", run.version).trim().to_string(), ended, when].into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" · ")
}

fn status(text: String, theme: &ThemeTokens) -> Div {
    div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)
}

/// The panel.
pub(super) fn render_run_panel(index: usize, panel: RunPanel, theme: &ThemeTokens) -> Stateful<Div> {
    let mut root = div().id((PANEL_ID, index)).w_full().px(theme.spacing_sm).py(theme.spacing_xs).border_t(theme.border_width_hairline).border_color(theme.border_subtle).flex().flex_col().gap(theme.spacing_3xs);
    if let Some((post, block, on_cancel)) = panel.running {
        let cancel = Button::new((CANCEL_ID, index)).ghost().xsmall().label(t!("code.output.cancel").to_string()).on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_cancel(post.clone(), block, window, cx);
        });
        root = root.child(div().flex().items_center().gap(theme.spacing_xs).child(status(t!("code.output.running").to_string(), theme)).child(cancel));
    }
    if let Some(run) = panel.saved {
        let notes = [Some(run_line(&run)), run.truncated.then(|| t!("code.output.truncated").to_string())].into_iter().flatten().collect::<Vec<_>>().join(" · ");
        let output = if run.output.trim().is_empty() { t!("code.output.empty").to_string() } else { run.output.trim_end().to_string() };
        let text = div().id((OUTPUT_ID, index)).max_h(theme.code_output_max_height).overflow_y_scroll().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(output);
        root = root.child(status(notes, theme)).child(text);
    }
    root
}

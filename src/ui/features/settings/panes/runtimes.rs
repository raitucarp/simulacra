//! Settings › Code › Runtimes (P1, P4): every runtime code blocks may run in, with the version
//! and place the last detection found and its own switch (usable once code running is on; a
//! runtime the last detection did not find has none), and Refresh, which asks each runtime its
//! version again. Nothing is probed until the user asks.

use super::super::controls::path::{home_dir, home_relative};
use super::common::{card, field_title, muted_note, section, section_label, spaced_row};
use crate::domain::code::{Runtime, RuntimeCache};
use crate::domain::preferences::CodePrefs;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::Sizable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const REFRESH_ID: &str = "settings-code-refresh";
const SWITCH_ID: &str = "settings-code-runtime-";

/// Something asked of the runtimes pane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeAction {
    /// Probe every runtime again.
    Refresh,
    /// P4: switch one runtime on (`true`) or off.
    Switch(Runtime, bool),
}

pub type RuntimeCallback = Rc<dyn Fn(RuntimeAction, &mut Window, &mut App) + 'static>;

/// What the pane shows.
#[derive(Clone)]
pub struct RuntimesProps {
    pub cache: RuntimeCache,
    /// P4: the master switch and the runtimes switched off.
    pub prefs: CodePrefs,
    /// A detection is running.
    pub detecting: bool,
    pub on_action: RuntimeCallback,
}

/// When the last detection ran, as the pane says it.
pub fn checked_label(cache: &RuntimeCache) -> String {
    match cache.probed_at {
        Some(at) => t!("code.runtimes.checked", time = crate::ui::features::data::format_time(at)).to_string(),
        None => t!("code.runtimes.never").to_string(),
    }
}

/// The RUNTIMES section.
pub fn render_runtimes(theme: &ThemeTokens, props: RuntimesProps) -> Div {
    let on_action = props.on_action.clone();
    let label = if props.detecting { t!("code.runtimes.checking") } else { t!("code.runtimes.refresh") };
    let refresh = Button::new(REFRESH_ID).ghost().xsmall().icon(LucideIcon::RefreshCw).label(label.to_string()).disabled(props.detecting).on_click(move |_, window, cx| on_action(RuntimeAction::Refresh, window, cx));
    let status = div().flex().items_center().gap(theme.spacing_sm).child(div().flex_1().child(muted_note(theme, checked_label(&props.cache)))).child(refresh);
    let mut body = card(theme).gap(theme.spacing_sm).child(muted_note(theme, t!("code.runtimes.note"))).child(status);
    for runtime in Runtime::ALL {
        body = body.child(runtime_row(theme, *runtime, &props));
    }
    section(spaced_row().child(section_label(theme, t!("code.runtimes.title"))), body, theme)
}

fn runtime_row(theme: &ThemeTokens, runtime: Runtime, props: &RuntimesProps) -> Div {
    let cache = &props.cache;
    let detail = match cache.get(runtime) {
        Some(found) => t!("code.runtimes.found", version = found.version.as_str(), path = home_relative(&found.path, home_dir().as_deref())).to_string(),
        None if cache.probed_at.is_some() => t!("code.runtimes.not_found").to_string(),
        None => String::new(),
    };
    let info = div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_2xs).child(field_title(theme, runtime.label().to_string())).child(muted_note(theme, detail));
    let on_action = props.on_action.clone();
    let switch = Switch::new(SharedString::from(format!("{SWITCH_ID}{}", runtime.key())))
        .checked(!props.prefs.is_switched_off(runtime))
        .color(theme.accent_brand)
        .disabled(!props.prefs.enabled)
        .on_click(move |on, window, cx| on_action(RuntimeAction::Switch(runtime, *on), window, cx));
    let missing = cache.probed_at.is_some() && cache.get(runtime).is_none();
    div().flex().items_center().gap(theme.spacing_sm).py(theme.spacing_2xs).child(info).when(!missing, |row| row.child(switch))
}

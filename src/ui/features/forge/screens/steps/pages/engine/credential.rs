//! Which stored key an engine reference uses: a readiness badge, one chip per matching
//! credential (plus "Automatic" / "None") and a shortcut to Settings › AI, where keys live.

use super::kit::engine_id;
use crate::agent::credentials::{Credential, CredentialKind};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{credential_status, CredentialStatus};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeEvent, ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{focus_ring_shadow, ActionCallback, CardGrid, GridCell};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const AUTO_KEY: &str = "forge.ui.engine.cred_auto";
const NONE_KEY: &str = "forge.ui.engine.cred_none";
const READY_KEY: &str = "forge.ui.engine.cred_ready";
const READY_AUTO_KEY: &str = "forge.ui.engine.cred_ready_auto";
const MISSING_KEY: &str = "forge.ui.engine.cred_missing";
const GONE_KEY: &str = "forge.ui.engine.cred_gone";
const OPTIONAL_KEY: &str = "forge.ui.engine.cred_optional";
const MANAGE_KEY: &str = "forge.ui.engine.cred_manage";
const ARG_LABEL: &str = "label";
const ARG_SERVICE: &str = "service";
/// Columns of the credential chips.
const CHIP_COLUMNS: usize = 2;

/// The badge text, icon and color of a credential status for `service`.
fn badge_parts(status: &CredentialStatus, service: &str, theme: &ThemeTokens) -> (String, LucideIcon, Rgba) {
    let service = (ARG_SERVICE, service.to_string());
    match status {
        CredentialStatus::Ready { label, chosen: true } => (tr_with(READY_KEY, &[(ARG_LABEL, label.clone())]), LucideIcon::CircleCheck, theme.accent_success),
        CredentialStatus::Ready { label, chosen: false } => (tr_with(READY_AUTO_KEY, &[(ARG_LABEL, label.clone())]), LucideIcon::CircleCheck, theme.accent_success),
        CredentialStatus::Missing => (tr_with(MISSING_KEY, &[service]), LucideIcon::CircleAlert, theme.warning),
        CredentialStatus::Gone => (tr(GONE_KEY), LucideIcon::CircleAlert, theme.accent_danger),
        CredentialStatus::NotNeeded => (tr(OPTIONAL_KEY), LucideIcon::Info, theme.text_muted),
    }
}

/// A pill stating whether requests can authenticate.
pub(super) fn readiness_badge(status: &CredentialStatus, service: &str, theme: &ThemeTokens) -> Div {
    let (text, icon, color) = badge_parts(status, service, theme);
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(color)
        .text_size(theme.font_size_xs)
        .text_color(color)
        .child(Icon::new(icon).size(theme.font_size_sm))
        .child(text)
}

/// One chip: a lock, the credential's name and its masked key.
fn chip(id: SharedString, (label, detail): (String, String), (selected, focused): (bool, bool), theme: &ThemeTokens, click: ActionCallback) -> Stateful<Div> {
    let border = if selected { theme.accent_brand } else { theme.border_subtle };
    let hover = theme.bg_elevated;
    div()
        .id(id)
        .flex_1()
        .min_w_0()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(border)
        .bg(if selected { theme.accent_brand_subtle } else { theme.bg_card })
        .cursor_pointer()
        .hover(move |style| style.bg(hover))
        .when(focused, |chip| chip.shadow(focus_ring_shadow(theme)))
        .on_click(move |_, w, cx| click(w, cx))
        .child(div().text_color(if selected { theme.accent_brand } else { theme.text_muted }).child(Icon::new(LucideIcon::Lock).size(theme.font_size_sm)))
        .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(label))
        .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(detail))
}

impl ForgeView {
    /// The key picker of one reference: `kind` = `None` means a key is optional (any stored
    /// credential may fill `{{api_key}}`).
    pub(super) fn credential_picker(
        &self,
        name: &str,
        (current, kind): (Option<&str>, Option<CredentialKind>),
        handle: &ForgeHandle,
        set: impl Fn(&mut EngineConfig, Option<String>) + 'static,
    ) -> Div {
        let theme = &self.theme;
        let store = &self.context.credentials;
        let status = credential_status(store, current, kind);
        let service = kind.map(CredentialKind::label).unwrap_or_default();
        let matching: Vec<&Credential> = store.items.iter().filter(|c| kind.is_none_or(|k| c.kind == k)).collect();
        let first = (tr(if kind.is_some() { AUTO_KEY } else { NONE_KEY }), String::new(), None);
        let options = std::iter::once(first).chain(matching.iter().map(|c| (c.label.clone(), c.masked_secret(), Some(c.id.clone()))));
        let pick = handle.engine(set);
        let cells = options
            .enumerate()
            .map(|(at, (label, detail, id))| {
                let selected = id.as_deref() == current;
                let (theme, pick) = (theme.clone(), pick.clone());
                let (cell_id, value) = (engine_id(format!("{name}-cred-{at}")), id.clone());
                let build = move |focused, click| chip(cell_id, (label, detail), (selected, focused), &theme, click);
                GridCell::new(selected, build, Rc::new(move |w, cx| pick(value.clone(), w, cx)))
            })
            .collect();
        let manage = handle.act(|_, _, cx| cx.emit(ForgeEvent::OpenAiSettings));
        let button = Button::new(engine_id(format!("{name}-manage"))).ghost().xsmall().icon(LucideIcon::Settings).label(tr(MANAGE_KEY));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_sm)
            .child(div().flex().flex_wrap().items_center().justify_between().gap(theme.spacing_xs).child(readiness_badge(&status, &service, theme)).child(button.on_click(move |_, w, cx| manage(w, cx))))
            .child(CardGrid::new(engine_id(format!("{name}-creds")), cells, CHIP_COLUMNS, theme.spacing_xs))
    }
}

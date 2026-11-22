//! The version row of the composer (V3): while a published post is edited, the version it
//! goes from and to, the bump (patch, minor, major) suggested by the changes, which the user
//! may override, and an optional edit note.

use crate::domain::data::history::{Bump, SemVer};
use crate::theme::ThemeTokens;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Selectable, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, Div, Entity, ParentElement, SharedString, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Picks the bump the edit makes.
pub type BumpCallback = Rc<dyn Fn(Bump, &mut Window, &mut App) + 'static>;

const BUMP_ID: &str = "composer-version-bump";

/// The row while a published post is edited.
#[derive(Clone)]
pub struct VersionView {
    /// The version edited from, and the one the edit makes.
    pub from: SemVer,
    pub next: SemVer,
    /// The bump the changes suggest, and the one the edit makes.
    pub suggested: Bump,
    pub chosen: Bump,
    /// The edit starts from an older version (a branch, V4).
    pub branch: bool,
    pub note: Entity<InputState>,
    pub on_pick: BumpCallback,
}

/// Locale key of a bump's name.
pub fn bump_label_key(bump: Bump) -> &'static str {
    match bump {
        Bump::Patch => "data.bump.patch",
        Bump::Minor => "data.bump.minor",
        Bump::Major => "data.bump.major",
    }
}

/// `v1.2.0`: a version as the app shows it.
pub fn version_label(version: SemVer) -> String {
    t!("data.version_label", version = version.to_string()).to_string()
}

fn bump_button(bump: Bump, view: &VersionView) -> Button {
    let on_pick = view.on_pick.clone();
    Button::new(SharedString::from(format!("{BUMP_ID}-{}", bump.key())))
        .ghost()
        .xsmall()
        .selected(view.chosen == bump)
        .label(t!(bump_label_key(bump)).to_string())
        .on_click(move |_, window, cx| on_pick(bump, window, cx))
}

fn muted(text: String, theme: &ThemeTokens) -> Div {
    div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)
}

/// The row: from → to, the bump buttons, the suggestion and the note field.
pub(super) fn version_row(view: &VersionView, theme: &ThemeTokens) -> Div {
    let step = t!("data.edit.version_step", from = version_label(view.from), to = version_label(view.next)).to_string();
    let numbers = div().flex_none().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_primary).child(step);
    let suggested = t!("data.edit.suggested", bump = t!(bump_label_key(view.suggested)).to_string()).to_string();
    let bumps = div().flex().items_center().gap(theme.spacing_2xs).children(Bump::ALL.map(|bump| bump_button(bump, view)));
    let line = div().flex().flex_wrap().items_center().gap(theme.control_gap).child(muted(t!("data.edit.version").to_string(), theme)).child(numbers).child(bumps).child(muted(suggested, theme));
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .px(theme.panel_padding)
        .py(theme.spacing_xs)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(line)
        .when(view.branch, |row| row.child(muted(t!("data.edit.branch_hint").to_string(), theme)))
        .child(Input::new(&view.note).small())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_bump_is_named_and_versions_read_with_a_v() {
        for key in Bump::ALL.map(bump_label_key).into_iter().chain(["data.edit.version", "data.edit.note_placeholder", "data.edit.branch_hint"]) {
            assert_ne!(t!(key, locale = "en"), key);
        }
        assert_eq!(t!("data.version_label", locale = "en", version = "1.2.0"), "v1.2.0");
    }
}

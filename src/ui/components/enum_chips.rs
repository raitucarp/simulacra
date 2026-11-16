//! Generic single-choice chip set over a fixed list of options (usually `Enum::ALL`).

use super::chip::{choice_chip, chip_row};
use crate::theme::ThemeTokens;
use gpui_kit::*;
use std::fmt::Debug;
use std::rc::Rc;

/// Separator between the id prefix and the per-option key in chip element ids.
const ID_SEPARATOR: &str = "-";

/// Declarative description of a chip set: options, current value, labels and element ids.
///
/// Element ids are `"{id_prefix}-{key(option)}"`; the key defaults to the option's
/// `Debug` representation and can be overridden with [`EnumChips::key`].
pub struct EnumChips<'a, T> {
    options: &'a [T],
    selected: Option<T>,
    id_prefix: &'a str,
    label: fn(T) -> &'static str,
    key: fn(T) -> String,
}

fn debug_key<T: Debug>(value: T) -> String {
    format!("{value:?}")
}

fn str_label(value: &'static str) -> &'static str {
    value
}

fn str_key(value: &'static str) -> String {
    value.to_string()
}

impl<'a> EnumChips<'a, &'static str> {
    /// Chips over plain string presets: each string is both its label and its id key,
    /// and the chip equal to `current` (if any) is selected.
    pub fn strings(options: &'a [&'static str], current: &str, id_prefix: &'a str) -> Self {
        let selected = options.iter().copied().find(|option| *option == current);
        Self { options, selected, id_prefix, label: str_label, key: str_key }
    }
}

impl<'a, T: Copy + PartialEq + Debug + 'static> EnumChips<'a, T> {
    pub fn new(
        options: &'a [T],
        selected: impl Into<Option<T>>,
        id_prefix: &'a str,
        label: fn(T) -> &'static str,
    ) -> Self {
        Self { options, selected: selected.into(), id_prefix, label, key: debug_key::<T> }
    }

    /// Overrides how each option is turned into the element-id suffix.
    pub fn key(mut self, key: fn(T) -> String) -> Self {
        self.key = key;
        self
    }

    /// Builds one chip per option without a wrapping container.
    pub fn chips(
        self,
        theme: &ThemeTokens,
        on_pick: impl Fn(T, &mut Window, &mut App) + 'static,
    ) -> Vec<Stateful<Div>> {
        let on_pick = Rc::new(on_pick);
        self.options
            .iter()
            .map(|&option| {
                let cb = on_pick.clone();
                choice_chip(
                    format!("{}{ID_SEPARATOR}{}", self.id_prefix, (self.key)(option)),
                    (self.label)(option),
                    self.selected == Some(option),
                    theme,
                    move |w, cx| cb(option, w, cx),
                )
            })
            .collect()
    }

    /// Builds the chips inside a wrapping [`chip_row`].
    pub fn render(
        self,
        theme: &ThemeTokens,
        on_pick: impl Fn(T, &mut Window, &mut App) + 'static,
    ) -> Div {
        chip_row(theme).children(self.chips(theme, on_pick))
    }
}

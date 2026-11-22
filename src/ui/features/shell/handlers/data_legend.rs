//! The legend page in the shell (S9): opened from the palette, it lists the core symbols and
//! the custom ones the user's posts carry; editing a custom symbol's meaning saves the key.

use super::super::view::AppView;
use crate::domain::symbols::SymbolRef;
use crate::storage::SettingKey;
use crate::ui::features::data::{DataEvent, LegendView};
use gpui_kit::*;

impl AppView {
    pub(in crate::ui::features::shell) fn on_legend_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::OpenLegend => self.data.legend_open = true,
            DataEvent::EditMeaning(symbol) => {
                let meaning = self.symbols.key.meaning_of(&symbol).unwrap_or_default().to_string();
                self.data.inputs.meaning.update(cx, |input, cx| {
                    input.set_value(meaning, window, cx);
                    input.focus(window, cx);
                });
                self.data.editing_meaning = Some(symbol);
            }
            DataEvent::SaveMeaning => self.save_meaning(cx),
            DataEvent::CancelMeaning => self.data.editing_meaning = None,
            _ => {
                self.data.legend_open = false;
                self.data.editing_meaning = None;
            }
        }
    }

    fn save_meaning(&mut self, cx: &mut Context<Self>) {
        let Some(symbol) = self.data.editing_meaning.take() else { return };
        let meaning = self.data.inputs.meaning.read(cx).value().to_string();
        if self.symbols.key.set(symbol, &meaning) {
            let (Some(db), Ok(json)) = (self.state.db.as_ref(), serde_json::to_string(&self.symbols.key)) else { return };
            if let Err(err) = db.set_setting(SettingKey::SymbolKey, &json) {
                eprintln!("simulacra: symbol key not saved: {err}");
            }
        }
    }

    /// The legend: every custom symbol in the user's posts or in the key, with its meaning.
    pub(in crate::ui::features::shell) fn legend_view(&self) -> LegendView {
        let mut custom: Vec<SymbolRef> = self.symbols.key.entries.iter().map(|entry| entry.symbol.clone()).collect();
        for symbol in self.state.posts().iter().flat_map(|post| post.extras.symbols.custom.iter()) {
            if !custom.contains(symbol) {
                custom.push(symbol.clone());
            }
        }
        let custom = custom.into_iter().map(|symbol| (symbol.clone(), self.symbols.key.meaning_of(&symbol).unwrap_or_default().to_string())).collect();
        LegendView { custom, editing: self.data.editing_meaning.clone(), meaning: self.data.inputs.meaning.clone() }
    }
}


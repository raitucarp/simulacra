//! Symbols wiring of the shell (epic S): what the composer's symbol picker remembers (S5),
//! saved in the vault's settings.

use super::view::AppView;
use crate::domain::symbols::{PostSymbols, SymbolMemory};
use crate::state::AppState;
use crate::storage::SettingKey;
use crate::ui::features::composer::ComposerModalPanel;
use gpui_kit::{Context, Entity};

/// Everything the symbols keep in the shell.
#[derive(Default)]
pub struct SymbolsHost {
    pub memory: SymbolMemory,
    /// What each custom symbol means (S9).
    pub key: crate::domain::symbols::SymbolKey,
}

impl SymbolsHost {
    /// Reads what the picker remembers and hands it to the composer.
    pub(super) fn new(state: &AppState, composer: &Entity<ComposerModalPanel>, cx: &mut Context<AppView>) -> Self {
        let saved = state.db.as_ref().and_then(|db| db.get_setting(SettingKey::SymbolPicker).ok().flatten());
        let memory = saved.and_then(|json| serde_json::from_str::<SymbolMemory>(&json).ok()).unwrap_or_default().validated();
        let given = memory.clone();
        composer.update(cx, |panel, _| panel.set_symbol_sources(given, std::rc::Rc::default()));
        Self { memory, key: load_symbol_key(state) }
    }
}

impl AppView {
    /// Remembers the symbols just published and saves the picker's memory.
    pub(super) fn remember_symbols(&mut self, used: &PostSymbols, cx: &mut Context<Self>) {
        if used.is_empty() {
            return;
        }
        self.symbols.memory = self.composer_modal.update(cx, |panel, _| panel.remember_symbols(used));
        let (Some(db), Ok(json)) = (self.state.db.as_ref(), serde_json::to_string(&self.symbols.memory)) else { return };
        if let Err(err) = db.set_setting(SettingKey::SymbolPicker, &json) {
            eprintln!("simulacra: symbol picker memory not saved: {err}");
        }
    }
}

/// The symbol key saved in the vault's settings (empty when none or unreadable, S9).
fn load_symbol_key(state: &AppState) -> crate::domain::symbols::SymbolKey {
    let saved = state.db.as_ref().and_then(|db| db.get_setting(SettingKey::SymbolKey).ok().flatten());
    saved.and_then(|json| serde_json::from_str::<crate::domain::symbols::SymbolKey>(&json).ok()).unwrap_or_default().validated()
}

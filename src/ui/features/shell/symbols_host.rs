//! Symbols wiring of the shell (epic S): what the composer's symbol picker remembers (S5),
//! saved in the vault's settings.

use super::view::AppView;
use crate::domain::symbols::{PostSymbols, SymbolMemory};
use crate::state::AppState;
use crate::storage::SettingKey;
use crate::ui::features::composer::ComposerModalPanel;
use gpui_kit::{Context, Entity};

/// Everything the symbols keep in the shell.
pub struct SymbolsHost {
    pub memory: SymbolMemory,
    /// What each custom symbol means (S9).
    pub key: crate::domain::symbols::SymbolKey,
    /// The icon set download under way, and the last failure (S13).
    pub download: Option<super::handlers::DownloadJob>,
    pub error: Option<String>,
    /// What the icon store holds (S13).
    pub installed: crate::domain::icons::StoreIndex,
    /// The installed icons by name (S15).
    pub index: std::rc::Rc<crate::domain::icons::SymbolIndex>,
}

impl SymbolsHost {
    /// Reads what the picker remembers and hands it to the composer.
    pub(super) fn new(state: &AppState, composer: &Entity<ComposerModalPanel>, cx: &mut Context<AppView>) -> Self {
        let saved = state.db.as_ref().and_then(|db| db.get_setting(SettingKey::SymbolPicker).ok().flatten());
        let memory = saved.and_then(|json| serde_json::from_str::<SymbolMemory>(&json).ok()).unwrap_or_default().validated();
        let (store, given) = (crate::icons::IconStore::new(state.symbols_dir()), memory.clone());
        let index = std::rc::Rc::new(symbol_index(&store));
        let library = index.clone();
        composer.update(cx, |panel, _| panel.set_symbol_sources(given, library));
        let key = load_symbol_key(state);
        crate::ui::features::symbols::set_symbol_meanings(&key); // S9: tooltips show the meanings
        Self { memory, key, download: None, error: None, installed: store.index(), index }
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

impl AppView {
    /// Reads what the icon store holds again (after a download or a removal, S13).
    pub(super) fn refresh_symbol_library(&mut self, cx: &mut Context<Self>) {
        let store = crate::icons::IconStore::new(self.state.symbols_dir());
        (self.symbols.installed, self.symbols.index) = (store.index(), std::rc::Rc::new(symbol_index(&store)));
        let (memory, library) = (self.symbols.memory.clone(), self.symbols.index.clone());
        self.composer_modal.update(cx, |panel, _| panel.set_symbol_sources(memory, library));
    }

    /// What Settings › Symbols shows (S13).
    pub(super) fn symbol_sets_props(&self, bind: &super::binder::Binder) -> crate::ui::features::settings::SymbolSetsProps {
        crate::ui::features::settings::SymbolSetsProps {
            installed: self.symbols.installed.sets.iter().map(|set| (set.id.clone(), set.bytes)).collect(),
            download: self.symbols.download.as_ref().map(|job| job.view()),
            error: self.symbols.error.clone(),
            on_action: std::rc::Rc::new(bind.with(Self::on_symbol_set_action)),
        }
    }

    /// The Licences screen: every downloaded set with the licence text stored with it.
    pub(super) fn licence_views(&self) -> Vec<crate::ui::features::settings::LicenceView> {
        let store = crate::icons::IconStore::new(self.state.symbols_dir());
        let sets = self.symbols.installed.sets.iter().filter_map(|stored| crate::domain::icons::icon_set(&stored.id));
        sets.map(|set| crate::ui::features::settings::LicenceView { set, text: store.licence(set.id).unwrap_or_default() }).collect()
    }
}

/// The name index of every installed icon (S15).
fn symbol_index(store: &crate::icons::IconStore) -> crate::domain::icons::SymbolIndex {
    let index = store.index();
    crate::domain::icons::SymbolIndex::build(index.sets.iter().map(|set| (set.id.as_str(), store.icon_names(&set.id))))
}

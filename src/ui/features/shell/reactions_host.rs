//! Reactions wiring of the shell (epic R): which card the emoji picker is open on, its
//! search field and tab, and what it remembers (recent emoji and skin tone, saved in the
//! vault's settings).

use super::view::AppView;
use crate::domain::emoji::PickerMemory;
use crate::domain::PostId;
use crate::state::AppState;
use crate::storage::SettingKey;
use crate::ui::features::reactions::PickerTab;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;

/// Everything the reactions keep in the shell.
pub struct ReactionsHost {
    /// The post whose picker is open.
    pub open: Option<PostId>,
    pub search: Entity<InputState>,
    pub tab: PickerTab,
    pub memory: PickerMemory,
}

impl ReactionsHost {
    /// Reads the picker's memory and creates its search field.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> Self {
        let saved = state.db.as_ref().and_then(|db| db.get_setting(SettingKey::EmojiPicker).ok().flatten());
        let memory = saved.and_then(|json| serde_json::from_str::<PickerMemory>(&json).ok()).unwrap_or_default().validated();
        let placeholder = t!("reactions.picker.search_placeholder").to_string();
        let search = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let tab = if memory.recent.is_empty() { PickerTab::Group(crate::domain::emoji::EmojiGroup::SmileysEmotion) } else { PickerTab::Recent };
        Self { open: None, search, tab, memory }
    }

    /// Saves what the picker remembers.
    pub(super) fn save_memory(&self, state: &AppState) {
        let (Some(db), Ok(json)) = (state.db.as_ref(), serde_json::to_string(&self.memory)) else { return };
        if let Err(err) = db.set_setting(SettingKey::EmojiPicker, &json) {
            eprintln!("simulacra: emoji picker memory not saved: {err}");
        }
    }
}

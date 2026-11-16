//! Persistence of [`ThemeSettings`] (one `theme.*` row per field) and the one-time migration
//! of the legacy `theme_tokens` row.

use super::{Database, SettingKey};
use crate::error::{AppError, Result};
use crate::theme::settings::ThemeSettings;
use crate::theme::{Density, RadiusScale, ThemeTokens, TypeScale};
use serde::de::DeserializeOwned;

fn to_json<T: serde::Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| AppError::DatabaseError(format!("theme settings: {e}")))
}

impl Database {
    /// Parses the JSON stored under `key`; absent or unreadable rows give `None`.
    fn setting_json<T: DeserializeOwned>(&self, key: SettingKey) -> Option<T> {
        let raw = self.get_setting(key).ok().flatten()?;
        serde_json::from_str(&raw)
            .map_err(|err| eprintln!("simulacra: ignoring unreadable `{}` setting: {err}", key.as_str()))
            .ok()
    }

    /// The setting under `key` mapped through `parse` (absent or unknown values give `None`).
    fn setting_with<T>(&self, key: SettingKey, parse: impl Fn(&str) -> Option<T>) -> Option<T> {
        self.get_setting(key).ok().flatten().and_then(|raw| parse(&raw))
    }

    /// Loads the theme settings, migrating (then removing) a legacy `theme_tokens` row.
    pub fn load_theme_settings(&self) -> ThemeSettings {
        let defaults = ThemeSettings::default();
        let mut settings = ThemeSettings {
            preset_id: self.setting_with(SettingKey::ThemePresetId, |id| Some(id.to_string())).unwrap_or(defaults.preset_id),
            overrides: self.setting_json(SettingKey::ThemeOverrides).unwrap_or_default(),
            custom: self.setting_json(SettingKey::ThemeCustomPalettes).unwrap_or_default(),
            density: self.setting_with(SettingKey::ThemeDensity, Density::from_id).unwrap_or_default(),
            type_scale: self.setting_with(SettingKey::ThemeTypeScale, TypeScale::from_id).unwrap_or_default(),
            radius_scale: self.setting_with(SettingKey::ThemeRadiusScale, RadiusScale::from_id).unwrap_or_default(),
            font_family: self
                .setting_with(SettingKey::ThemeFontFamily, |f| (!f.trim().is_empty()).then(|| f.to_string()))
                .unwrap_or(defaults.font_family),
        };
        self.migrate_legacy_theme(&mut settings);
        settings
    }

    /// Folds a legacy `theme_tokens` row into `settings`, persists the result and drops the row.
    fn migrate_legacy_theme(&self, settings: &mut ThemeSettings) {
        if self.get_setting(SettingKey::LegacyThemeTokens).ok().flatten().is_none() {
            return;
        }
        if let Some(tokens) = self.setting_json::<ThemeTokens>(SettingKey::LegacyThemeTokens) {
            settings.adopt_legacy_tokens(&tokens);
        }
        let migrated = self.save_theme_settings(settings).and_then(|()| self.delete_setting(SettingKey::LegacyThemeTokens));
        if let Err(err) = migrated {
            eprintln!("simulacra: legacy theme migration not saved: {err}");
        }
    }

    /// Persists every theme setting.
    pub fn save_theme_settings(&self, settings: &ThemeSettings) -> Result<()> {
        let rows = [
            (SettingKey::ThemePresetId, settings.preset_id.clone()),
            (SettingKey::ThemeOverrides, to_json(&settings.overrides)?),
            (SettingKey::ThemeCustomPalettes, to_json(&settings.custom)?),
            (SettingKey::ThemeDensity, settings.density.id().to_string()),
            (SettingKey::ThemeTypeScale, settings.type_scale.id().to_string()),
            (SettingKey::ThemeRadiusScale, settings.radius_scale.id().to_string()),
            (SettingKey::ThemeFontFamily, settings.font_family.clone()),
        ];
        rows.iter().try_for_each(|(key, value)| self.set_setting(*key, value))
    }
}

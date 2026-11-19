//! Generators of the per-category field enums and of the [`SettingField`](super::SettingField)
//! dispatch over them.

/// One category's field enum with `ALL` and stable `key()`s.
macro_rules! category_fields {
    ($(#[$doc:meta])* $name:ident { $($variant:ident => $key:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            /// Every field, in display order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Stable key inside the category (element ids and i18n keys).
            pub const fn key(self) -> &'static str {
                match self {
                    $(Self::$variant => $key,)+
                }
            }
        }
    };
}

/// [`SettingField`](super::SettingField), [`PreferenceCategory`](super::PreferenceCategory)
/// and their dispatch from `Variant(FieldEnum) in prefs_field => "key"` rows.
macro_rules! setting_fields {
    ($($variant:ident($field:ident) in $prefs:ident => $key:literal),+ $(,)?) => {
        /// A category that owns editable preferences.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PreferenceCategory {
            $($variant,)+
        }

        impl PreferenceCategory {
            /// Every category, in settings order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Stable key (`general`, `agents`, ...).
            pub const fn key(self) -> &'static str {
                match self {
                    $(Self::$variant => $key,)+
                }
            }
        }

        /// Any editable preference.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum SettingField {
            $($variant($field),)+
        }

        impl SettingField {
            /// Every field of every category, in settings order.
            pub fn all() -> Vec<Self> {
                std::iter::empty()
                    $(.chain($field::ALL.iter().map(|field| Self::$variant(*field))))+
                    .collect()
            }

            /// The category owning this field.
            pub const fn category(self) -> PreferenceCategory {
                match self {
                    $(Self::$variant(_) => PreferenceCategory::$variant,)+
                }
            }

            /// Stable key inside the category.
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant(field) => field.key(),)+
                }
            }

            /// The control this field is edited with.
            pub fn kind(self) -> FieldKind {
                match self {
                    $(Self::$variant(field) => field.kind(),)+
                }
            }

            /// The field's current value in `prefs`.
            pub fn get(self, prefs: &Preferences) -> SettingValue {
                match self {
                    $(Self::$variant(field) => field.get(&prefs.$prefs),)+
                }
            }

            /// `prefs` with this field set to `value`, then validated (clamped).
            pub fn set(self, prefs: Preferences, value: &SettingValue) -> Preferences {
                let next = match self {
                    $(Self::$variant(field) => Preferences { $prefs: field.set(prefs.$prefs, value), ..prefs },)+
                };
                next.validated()
            }
        }
    };
}

pub(super) use {category_fields, setting_fields};

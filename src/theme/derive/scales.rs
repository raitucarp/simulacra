//! User-selectable scales applied on top of a palette: spacing density, type size and radii.

use serde::{Deserialize, Serialize};

/// Generates a three-step scale enum with its variants, default, factors and ids.
macro_rules! scale_enum {
    (
        $(#[$doc:meta])* $name:ident, default: $default:ident,
        [$($variant:ident => $id:literal, $factor:expr);* $(;)?]
    ) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            /// Every step, smallest first.
            pub const ALL: &'static [$name] = &[$($name::$variant,)*];

            /// Multiplier applied to the base metric.
            pub const fn factor(self) -> f32 {
                match self {
                    $($name::$variant => $factor,)*
                }
            }

            /// Stable identifier (persisted value and i18n suffix).
            pub const fn id(self) -> &'static str {
                match self {
                    $($name::$variant => $id,)*
                }
            }

            /// Parses a persisted identifier.
            pub fn from_id(id: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|step| step.id() == id)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default
            }
        }
    };
}

scale_enum! {
    /// Spacing density of the whole interface.
    Density, default: Comfortable,
    [Compact => "compact", 0.75; Comfortable => "comfortable", 1.0; Cozy => "cozy", 1.25]
}

scale_enum! {
    /// Text size scale.
    TypeScale, default: Default,
    [Small => "small", 0.92; Default => "default", 1.0; Large => "large", 1.1]
}

scale_enum! {
    /// Corner radius scale (pills stay fully round).
    RadiusScale, default: Default,
    [Sharp => "sharp", 0.35; Default => "default", 1.0; Round => "round", 1.6]
}

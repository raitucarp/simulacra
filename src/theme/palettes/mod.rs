//! Theme palettes for customizable network appearance.
//! Defines tokenized presets (Threads OLED Black, Obsidian Deep Zinc, Midnight Slate).

/// Assigns a table of `field: value` pairs onto a mutable [`ThemeTokens`].
macro_rules! set_tokens {
    ($theme:ident { $($field:ident: $value:expr),* $(,)? }) => {
        $($theme.$field = $value;)*
    };
}

mod common;
mod surfaces;

use super::tokens::ThemeTokens;

/// Shared typography, spacing, layout and semantic tokens common to every preset.
fn common_tokens() -> ThemeTokens {
    let mut t = ThemeTokens::blank();
    common::apply_typography(&mut t);
    common::apply_layout(&mut t);
    common::apply_semantic_colors(&mut t);
    t
}

/// True Threads / Linear Minimalist OLED Black aesthetic (Default).
pub fn minimalist_dark() -> ThemeTokens {
    let mut t = common_tokens();
    surfaces::apply_oled_black(&mut t);
    t
}

/// Obsidian Deep Zinc palette.
pub fn obsidian_zinc() -> ThemeTokens {
    let mut t = minimalist_dark();
    surfaces::apply_obsidian_zinc(&mut t);
    t
}

/// Midnight Slate oceanic palette.
pub fn midnight_slate() -> ThemeTokens {
    let mut t = minimalist_dark();
    surfaces::apply_midnight_slate(&mut t);
    t
}

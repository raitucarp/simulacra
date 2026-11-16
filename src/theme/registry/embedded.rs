//! Compile-time table of every bundled palette JSON (no runtime file IO).

/// Embeds `assets/themes/<id>.json` for each listed id, in display order.
macro_rules! embedded_palettes {
    ($($id:literal),* $(,)?) => {
        /// `(id, json)` of every bundled palette, in gallery order.
        pub(super) const EMBEDDED_PALETTES: &[(&str, &str)] = &[
            $(($id, include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/themes/", $id, ".json"))),)*
        ];
    };
}

embedded_palettes![
    // Simulacra house palettes
    "simulacra-midnight", "obsidian-zinc", "midnight-slate",
    // Dark community palettes
    "ayu-mirage", "catppuccin-frappe", "catppuccin-macchiato", "catppuccin-mocha", "dracula",
    "everforest", "github-dark", "gruvbox-dark", "kanagawa", "monokai", "night-owl", "nord",
    "one-dark", "rose-pine", "solarized-dark", "synthwave-84", "tokyo-night",
    // Light community palettes
    "ayu-light", "catppuccin-latte", "github-light", "gruvbox-light", "one-light",
    "rose-pine-dawn", "solarized-light", "tokyo-night-day",
];

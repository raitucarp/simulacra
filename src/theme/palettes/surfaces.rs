//! Per-preset surface, border, text and accent colors.

use crate::theme::ThemeTokens;
use gpui_kit::{rgb, rgba};

/// Pure OLED black surfaces with violet brand accents (AGENTS.md defaults).
pub(super) fn apply_oled_black(t: &mut ThemeTokens) {
    set_tokens!(t {
        bg_app: rgb(0x000000),          // Pure OLED black background
        bg_card: rgb(0x08080a),         // Layered L1 card surface (AGENTS.md #08080a)
        bg_elevated: rgb(0x111115),     // Surface L2 elevated/flyout (AGENTS.md #111115)
        bg_sidebar: rgb(0x000000),      // Pure OLED black sidebar
        bg_callout: rgba(0x8b5cf612),   // Subtle violet tinted quote box
        hover_bg: rgba(0xffffff0a),     // Clean ghost hover
        border: rgb(0x1e1e22),          // Precision hairline neutral border
        border_subtle: rgb(0x18181b),   // Subtle threadline and card divider
        border_strong: rgb(0x27272a),   // Focused inputs and card outlines
        border_focus: rgb(0x8b5cf6),    // Dialectic brand focus ring
        text_primary: rgb(0xf4f4f5),    // Crisp high-legibility pure white
        text_secondary: rgb(0xa1a1aa),  // Balanced neutral zinc secondary
        text_muted: rgb(0x71717a),      // Threads neutral muted metadata
        accent_user: rgb(0x38bdf8),     // Clean sky accent
        accent_alt: rgb(0xeab308),      // Dialectic grounding gold
        accent_agent: rgb(0xa855f7),    // Sophisticated dialectic purple
        accent_brand: rgb(0x8b5cf6),    // Brand violet
        accent_dialectic: rgb(0xeab308),// Grounding highlight
        accent_success: rgb(0x10b981),  // Active loop emerald
    });
}

/// Obsidian Deep Zinc overrides.
pub(super) fn apply_obsidian_zinc(t: &mut ThemeTokens) {
    set_tokens!(t {
        bg_app: rgb(0x09090b), bg_card: rgb(0x121215), bg_elevated: rgb(0x18181b),
        bg_sidebar: rgb(0x09090b), bg_callout: rgba(0xa855f715), hover_bg: rgba(0xffffff0a),
        border: rgb(0x27272a), border_subtle: rgb(0x1f1f23), border_strong: rgb(0x3f3f46),
        border_focus: rgb(0xa855f7),
        text_primary: rgb(0xf4f4f5), text_secondary: rgb(0xa1a1aa), text_muted: rgb(0x71717a),
        accent_user: rgb(0x38bdf8), accent_alt: rgb(0xeab308), accent_agent: rgb(0xa855f7),
        accent_brand: rgb(0xa855f7), accent_dialectic: rgb(0xeab308),
        accent_success: rgb(0x10b981),
        accent_brand_subtle: rgba(0xa855f720), accent_brand_border: rgba(0xa855f72a),
        bg_feed_post: rgb(0x000000),
    });
}

/// Midnight Slate oceanic overrides.
pub(super) fn apply_midnight_slate(t: &mut ThemeTokens) {
    set_tokens!(t {
        bg_app: rgb(0x0a0e17), bg_card: rgb(0x0f172a), bg_elevated: rgb(0x1e293b),
        bg_sidebar: rgb(0x070a10), bg_callout: rgba(0x38bdf812), hover_bg: rgba(0xffffff0a),
        border: rgb(0x1e293b), border_subtle: rgb(0x162032), border_strong: rgb(0x334155),
        border_focus: rgb(0x38bdf8),
        text_primary: rgb(0xf8fafc), text_secondary: rgb(0x94a3b8), text_muted: rgb(0x64748b),
        accent_user: rgb(0x38bdf8), accent_alt: rgb(0xf59e0b), accent_agent: rgb(0x818cf8),
        accent_brand: rgb(0x38bdf8), accent_dialectic: rgb(0xf59e0b),
        accent_success: rgb(0x10b981),
        accent_brand_subtle: rgba(0x38bdf820), accent_brand_border: rgba(0x38bdf82a),
        accent_dialectic_subtle: rgba(0xf59e0b20), accent_dialectic_border: rgba(0xf59e0b40),
        bg_feed_post: rgb(0x000000),
    });
}

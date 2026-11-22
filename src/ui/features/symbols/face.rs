//! A post's symbols drawn in place of its avatar (S4): the primary symbol at avatar size,
//! with up to two signifier badges overlapping its lower corner, and the names of all of
//! them in its tooltip. Feed and inspector cards show it in the avatar column, outline
//! bullets show the primary symbol small.

use super::labels::{art_label, glyph_label};
use crate::domain::symbols::{SymbolArt, SymbolFace, SymbolGlyph};
use crate::theme::ThemeTokens;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;

/// Element id prefix of a face (suffixed with the post id).
const FACE_ID_PREFIX: &str = "symbol-face-";
/// Separator of the names in a face's tooltip.
const NAME_SEPARATOR: &str = ", ";

/// The core art of `art`, `size` wide.
pub fn render_symbol_art(art: SymbolArt, size: Pixels) -> Illustration {
    Illustration::asset(art.path(), IllustrationKind::Symbol).width(size)
}

/// One glyph, `size` wide: core art, or a library icon on a disc in the text colour.
pub fn render_glyph(glyph: &SymbolGlyph, size: Pixels, theme: &ThemeTokens) -> AnyElement {
    match glyph {
        SymbolGlyph::Core(art) => render_symbol_art(*art, size).into_any_element(),
        SymbolGlyph::Custom(symbol) => div()
            .size(size)
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.radius_full)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(svg().path(SharedString::from(symbol.asset_path())).size(size * theme.symbol_glyph_scale).text_color(theme.text_primary))
            .into_any_element(),
    }
}

/// The face of post `post_id` at avatar `size`.
pub fn render_symbol_face(post_id: &str, face: &SymbolFace, size: Pixels, theme: &ThemeTokens) -> Stateful<Div> {
    let names: Vec<String> = std::iter::once(glyph_label(&face.primary)).chain(face.badges.iter().map(|art| art_label(*art))).collect();
    let tooltip = names.join(NAME_SEPARATOR);
    let badges = face.badges.iter().map(|art| div().rounded(theme.radius_full).bg(theme.bg_app).child(render_symbol_art(*art, theme.symbol_badge_size)));
    let corner = div().absolute().right(-theme.symbol_badge_offset).bottom(-theme.symbol_badge_offset).flex().gap(theme.spacing_3xs).children(badges);
    div()
        .id(SharedString::from(format!("{FACE_ID_PREFIX}{post_id}")))
        .relative()
        .flex_none()
        .size(size)
        .child(render_glyph(&face.primary, size, theme))
        .child(corner)
        .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
}

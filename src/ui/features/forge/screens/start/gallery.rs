//! The palette gallery: illustrated cards of the full palettes on the chosen shelf that match
//! the search; hovering a card, or moving the keyboard cursor onto it, previews it.

use super::super::common::muted;
use crate::domain::palette::{Palette, PaletteCategory};
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{CardGrid, GridCell, OptionCardSpec};
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::*;

/// Palette cards per row until the grid's width is first measured.
const GALLERY_COLUMNS: usize = 4;
const CARD_ID_PREFIX: &str = "forge-palette";
const GALLERY_GRID_ID: &str = "forge-palette-grid";
const NO_RESULTS_KEY: &str = "forge.palette.ui.no_results";
const SEARCH_ALL_KEY: &str = "forge.palette.ui.search_all";
const ARG_QUERY: &str = "query";

/// Whether `palette` matches the (lower-cased, trimmed) search `query`.
fn matches(palette: &Palette, query: &str) -> bool {
    query.is_empty()
        || [palette.label_key(), palette.pitch_key()].iter().any(|key| tr(key).to_lowercase().contains(query))
        || palette.key.replace('_', " ").contains(query)
}

/// Full palettes matching `search`: a non-empty query searches every shelf, an empty one shows
/// the `shelf` (`None`: all), in gallery order.
pub(super) fn gallery_matches(shelf: Option<PaletteCategory>, search: &str) -> Vec<&'static Palette> {
    let query = search.trim().to_lowercase();
    let shelf = if query.is_empty() { shelf } else { None };
    Palette::full(shelf).filter(|palette| matches(palette, &query)).collect()
}

/// The illustration of a full palette (class-sized art).
pub(super) fn palette_art(palette: &Palette) -> Illustration {
    Illustration::asset(palette.illustration(), IllustrationKind::Class)
}

impl ForgeView {
    /// Full palettes the gallery shows for the current shelf and search.
    pub(super) fn gallery_palettes(&self) -> Vec<&'static Palette> {
        gallery_matches(self.start.category, &self.start.search)
    }

    /// The card grid, or a line saying nothing matches the search.
    pub(super) fn palette_grid(&self, handle: &ForgeHandle) -> AnyElement {
        let palettes = self.gallery_palettes();
        let query = self.start.search.trim().to_string();
        if palettes.is_empty() {
            return muted(tr_with(NO_RESULTS_KEY, &[(ARG_QUERY, query)]), &self.theme).into_any_element();
        }
        let cells = palettes.into_iter().map(|palette| self.palette_card(palette, handle)).collect();
        let grid = CardGrid::new(GALLERY_GRID_ID, cells, GALLERY_COLUMNS, self.theme.stack_gap).auto_columns(self.theme.forge_class_card_width);
        let note = (!query.is_empty() && self.start.category.is_some()).then(|| muted(tr_with(SEARCH_ALL_KEY, &[(ARG_QUERY, query)]), &self.theme));
        div().flex().flex_col().gap(self.theme.stack_gap).children(note).child(grid).into_any_element()
    }

    fn palette_card(&self, palette: &'static Palette, handle: &ForgeHandle) -> GridCell {
        let theme = &self.theme;
        let width = theme.forge_class_card_width;
        let id = SharedString::from(format!("{CARD_ID_PREFIX}-{}", palette.key));
        let mut spec = OptionCardSpec::new(id, palette_art(palette).width(width), tr(&palette.label_key()), width);
        spec.description = Some(tr(&palette.pitch_key()));
        spec.selected = self.start.preview.is_some_and(|shown| shown.key == palette.key);
        let pick = handle.act(move |this, _, cx| this.preview_palette(palette, true, cx));
        let hover = handle.act(move |this, _, cx| this.preview_palette(palette, false, cx));
        GridCell::card(spec, theme, pick).on_hover(hover)
    }

    /// Shows `palette` in the preview panel. A click (`force`) also closes the roll and
    /// describe panels; a hover never replaces them.
    pub(crate) fn preview_palette(&mut self, palette: &'static Palette, force: bool, cx: &mut Context<Self>) {
        let side_busy = self.start.rolled.is_some() || self.start.describing;
        let shown = self.start.preview.is_some_and(|p| p.key == palette.key);
        if (force || !side_busy) && (!shown || side_busy) {
            self.start.preview = Some(palette);
            self.start.rolled = None;
            self.start.describing = false;
            cx.notify();
        }
    }
}

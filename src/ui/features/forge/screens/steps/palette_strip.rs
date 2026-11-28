//! The step palette strip above a step's options: illustrated chips of the palettes that only
//! touch this step. Hovering (or moving the keyboard cursor onto) a chip previews what it would
//! change; clicking it applies it as one undo step. The strip is folded to its title line until
//! asked for, so the step's own fields start near the top of the panel.

use super::super::common::muted;
use super::super::start::step_group;
use crate::domain::palette::Palette;
use crate::state::forge::{preview_palette, step_palettes, FieldChange};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{CardGrid, GridCell, OptionCardSpec};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const STRIP_ID: &str = "forge-palette-strip";
const STRIP_AREA_ID: &str = "forge-palette-strip-area";
const STRIP_SCROLL_ID: &str = "forge-palette-strip-scroll";
const STRIP_TOGGLE_ID: &str = "forge-palette-strip-toggle";
const CHIP_ID_PREFIX: &str = "forge-step-palette";
const TITLE_KEY: &str = "forge.palette.ui.strip_title";
const HINT_KEY: &str = "forge.palette.ui.strip_hint";
const NO_CHANGES_KEY: &str = "forge.palette.ui.no_changes";

impl ForgeView {
    /// The current step's palette strip, when the step has palettes.
    pub(super) fn palette_strip(&self, handle: &ForgeHandle) -> Option<Div> {
        let theme = &self.theme;
        let step = self.draft.step();
        let palettes = step_palettes(step);
        if palettes.is_empty() {
            return None;
        }
        let cells: Vec<GridCell> = palettes.iter().map(|palette| self.strip_chip(palette, handle)).collect();
        let columns = cells.len();
        let leave = handle.act(|this, _, cx| this.preview_step_palette(None, cx));
        let grid = div()
            .id(STRIP_SCROLL_ID)
            .w_full()
            .overflow_x_scrollbar()
            .pb(theme.spacing_xs)
            .child(CardGrid::new(STRIP_ID, cells, columns, theme.spacing_sm));
        let hovered = self.strip_preview.filter(|p| palettes.iter().any(|shown| shown.key == p.key));
        let open = self.strip_open;
        Some(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_sm)
                .p(theme.card_padding)
                .rounded(theme.radius_lg)
                .border(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .bg(theme.bg_card)
                .child(div().flex().child(self.strip_toggle(palettes.len(), handle)))
                .when(open, |strip| {
                    strip
                        .child(muted(tr(HINT_KEY), theme))
                        .child(div().id(STRIP_AREA_ID).on_hover(move |inside, window, cx| if !*inside { leave(window, cx) }).child(grid))
                        .children(hovered.map(|palette| self.strip_changes(palette)))
                }),
        )
    }

    /// The title line, folding or unfolding the strip (a button, so Tab reaches it).
    fn strip_toggle(&self, count: usize, handle: &ForgeHandle) -> Button {
        let chevron = if self.strip_open { LucideIcon::ChevronDown } else { LucideIcon::ChevronRight };
        let toggle = handle.act(|this, _, cx| {
            this.strip_open = !this.strip_open;
            cx.notify();
        });
        Button::new(STRIP_TOGGLE_ID)
            .ghost()
            .small()
            .icon(chevron)
            .label(format!("{} ({count})", tr(TITLE_KEY)))
            .on_click(move |_, window, cx| toggle(window, cx))
    }

    fn strip_chip(&self, palette: &'static Palette, handle: &ForgeHandle) -> GridCell {
        let theme = &self.theme;
        let width = theme.forge_palette_chip_width;
        let art = Illustration::asset(palette.illustration(), IllustrationKind::TraitCard).width(width);
        let id = SharedString::from(format!("{CHIP_ID_PREFIX}-{}", palette.key));
        let mut spec = OptionCardSpec::new(id, art, tr(&palette.label_key()), width);
        spec.description = Some(tr(&palette.pitch_key()));
        spec.selected = self.strip_preview.is_some_and(|p| p.key == palette.key);
        let apply = handle.act(move |this, window, cx| this.apply_step_palette(palette, window, cx));
        let hover = handle.act(move |this, _, cx| this.preview_step_palette(Some(palette), cx));
        GridCell::card(spec, theme, apply).on_hover(hover)
    }

    /// What the hovered strip palette would change on this step.
    fn strip_changes(&self, palette: &Palette) -> Div {
        let changes = preview_palette(&self.draft, palette);
        let rows: Vec<&FieldChange> = changes.iter().collect();
        if rows.is_empty() {
            return muted(tr(NO_CHANGES_KEY), &self.theme);
        }
        step_group(self.draft.step(), &rows, &self.theme)
    }
}

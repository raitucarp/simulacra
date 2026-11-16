//! The class card grid and the "other ways to begin" tiles.

use crate::domain::ArchetypeClass;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ActionCallback, CardGrid, GridCell, KeyboardStop, OptionCardSpec};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{ClassKey, Illustration};
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Class cards per row.
const CLASS_COLUMNS: usize = 4;
const CLASS_ID_PREFIX: &str = "forge-class";
const CLASS_GRID_ID: &str = "forge-class-grid";
const SCRATCH_ID: &str = "forge-tile-scratch";
const ROLL_ID: &str = "forge-tile-roll";
const DESCRIBE_ID: &str = "forge-tile-describe";
const SCRATCH_TITLE: &str = "forge.ui.start.scratch_title";
const SCRATCH_BODY: &str = "forge.ui.start.scratch_body";
const ROLL_TITLE: &str = "forge.ui.start.roll_title";
const ROLL_BODY: &str = "forge.ui.start.roll_body";
const DESCRIBE_TITLE: &str = "forge.ui.start.describe_title";
const DESCRIBE_BODY: &str = "forge.ui.start.describe_body";

/// The illustration key of a class (keys are 1:1 with `ArchetypeClass` keys).
pub(super) fn class_key(class: ArchetypeClass) -> ClassKey {
    ClassKey::ALL.iter().copied().find(|key| key.key() == class.key()).unwrap_or(ClassKey::SocraticGadfly)
}

impl ForgeView {
    /// The class cards; hovering a card, or moving the keyboard cursor onto it, previews it.
    pub(super) fn class_grid(&self, handle: &ForgeHandle) -> CardGrid {
        let cells = ArchetypeClass::ALL.iter().map(|class| self.class_card(*class, handle)).collect();
        CardGrid::new(CLASS_GRID_ID, cells, CLASS_COLUMNS, self.theme.spacing_md)
    }

    fn class_card(&self, class: ArchetypeClass, handle: &ForgeHandle) -> GridCell {
        let theme = &self.theme;
        let width = theme.forge_class_card_width;
        let art = Illustration::class(class_key(class)).width(width);
        let mut spec = OptionCardSpec::new(SharedString::from(format!("{CLASS_ID_PREFIX}-{}", class.key())), art, tr(&class.label_key()), width);
        spec.description = Some(tr(&class.pitch_key()));
        spec.selected = self.start.preview == Some(class);
        let pick = handle.act(move |this, _, cx| this.preview_class(class, true, cx));
        let hover = handle.act(move |this, _, cx| this.preview_class(class, false, cx));
        GridCell::card(spec, theme, pick).on_hover(hover)
    }

    pub(super) fn begin_tiles(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .gap(theme.spacing_md)
            .child(tile(SCRATCH_ID, LucideIcon::Plus, SCRATCH_TITLE, SCRATCH_BODY, theme, handle.act(ForgeView::begin_blank)))
            .child(tile(ROLL_ID, LucideIcon::Dices, ROLL_TITLE, ROLL_BODY, theme, handle.act(ForgeView::roll)))
            .child(tile(DESCRIBE_ID, LucideIcon::Sparkles, DESCRIBE_TITLE, DESCRIBE_BODY, theme, handle.act(ForgeView::toggle_describe)))
    }
}

/// One "way to begin" tile: icon, title and a line of explanation.
fn tile(id: &'static str, icon: LucideIcon, title_key: &str, body_key: &str, theme: &ThemeTokens, on_click: ActionCallback) -> Stateful<Div> {
    let hover_bg = theme.bg_elevated;
    let click = on_click.clone();
    div()
        .id(id)
        .flex_1()
        .min_h(theme.forge_tile_min_height)
        .p(theme.spacing_lg)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .rounded(theme.radius_xl)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_card)
        .cursor_pointer()
        .hover(move |style| style.bg(hover_bg))
        .on_click(move |_, window, cx| click(window, cx))
        .keyboard_stop(theme, on_click)
        .child(div().text_color(theme.accent_brand).child(Icon::new(icon).size(theme.font_size_xl)))
        .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).child(tr(title_key)))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(body_key)))
}

impl ForgeView {
    /// Shows `class` in the preview panel. A click (`force`) also closes the roll and
    /// describe panels; a hover never replaces them.
    pub(crate) fn preview_class(&mut self, class: ArchetypeClass, force: bool, cx: &mut Context<Self>) {
        let side_busy = self.start.rolled.is_some() || self.start.describing;
        if (force || !side_busy) && (self.start.preview != Some(class) || side_busy) {
            self.start.preview = Some(class);
            self.start.rolled = None;
            self.start.describing = false;
            cx.notify();
        }
    }
}

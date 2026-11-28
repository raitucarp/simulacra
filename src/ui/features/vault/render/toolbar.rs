//! The browse toolbar: search, the facet drawer button (compact), sort and the
//! shelf / table switch.

use crate::domain::document::query::SortKey;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::BrowseLayout;
use crate::ui::features::vault::text::sort_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const SORT_BUTTON_ID: &str = "vault-sort";
const FILTERS_BUTTON_ID: &str = "vault-filters";
const SHELF_SEGMENT_ID: &str = "vault-layout-shelf";
const TABLE_SEGMENT_ID: &str = "vault-layout-table";

impl VaultPanel {
    pub(super) fn render_toolbar(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let search = div()
            .flex_1()
            .min_w_0()
            .max_w(theme.vault_search_width)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.input_bg)
            .px(theme.spacing_sm)
            .child(
                Input::new(&self.search_input)
                    .bordered(false)
                    .cleanable(true)
                    .prefix(Icon::new(LucideIcon::Search).size(theme.icon_size_sm).text_color(theme.text_muted)),
            );
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(search)
            .when(compact, |bar| bar.child(self.render_filters_button(cx)))
            .child(
                div()
                    .ml_auto()
                    .flex()
                    .items_center()
                    .gap(theme.control_gap)
                    .child(self.render_sort_menu(compact, cx))
                    .child(self.render_layout_switch(cx)),
            )
    }

    fn render_filters_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let chosen = self.query.selection.len();
        let label = if chosen == 0 { t!("vault.filters").to_string() } else { t!("vault.filters_count", count = chosen).to_string() };
        Button::new(FILTERS_BUTTON_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Filter))
            .label(label)
            .on_click(cx.listener(|this, _, _, cx| this.set_drawer(true, cx)))
    }

    fn render_sort_menu(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let (current, descending) = self.query.effective_sort();
        let panel = cx.entity().downgrade();
        let label = t!("vault.sort_by", key = sort_label(current)).to_string();
        Button::new(SORT_BUTTON_ID)
            .ghost()
            .small()
            .icon(Icon::new(if descending { LucideIcon::ArrowDown } else { LucideIcon::ArrowUp }))
            .tooltip(label.clone())
            .when(!compact, |b| b.label(label))
            .dropdown_menu(move |menu, window, cx| render_menu(menu, &sort_menu_spec(current, descending, &panel), window, cx))
    }

    fn render_layout_switch(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let segment = |id: &'static str, icon, label: String, layout: BrowseLayout| Segment {
            id: id.into(),
            icon,
            label,
            active: self.layout == layout,
            active_icon_color: theme.accent_brand,
        };
        let shelf = segment(SHELF_SEGMENT_ID, LucideIcon::LayoutGrid, t!("vault.layout_shelf").to_string(), BrowseLayout::Shelf);
        let table = segment(TABLE_SEGMENT_ID, LucideIcon::List, t!("vault.layout_table").to_string(), BrowseLayout::Table);
        // A content-wide track: its pills keep their natural width (shared-width pills, the
        // segment default, measured nothing here and cut "Table" to "Ta…").
        segmented_track(theme)
            .w_auto()
            .flex_shrink_0()
            .child(segment_pill(shelf, theme, self.layout_listener(BrowseLayout::Shelf, cx)).flex_none())
            .child(segment_pill(table, theme, self.layout_listener(BrowseLayout::Table, cx)).flex_none())
    }

    fn layout_listener(&self, layout: BrowseLayout, cx: &mut Context<Self>) -> impl Fn(&mut Window, &mut App) + 'static {
        let panel = cx.entity().downgrade();
        move |_, cx| {
            let _ = panel.update(cx, |this, cx| this.set_layout(layout, cx));
        }
    }
}

/// Sort by (every key, the current one ticked; choosing it again still reverses it) and
/// Order (ascending or descending, the current direction ticked).
fn sort_menu_spec(current: SortKey, descending: bool, panel: &WeakEntity<VaultPanel>) -> MenuSpec {
    let by = |key: SortKey| {
        let panel = panel.clone();
        move |_: &mut Window, cx: &mut App| {
            let _ = panel.update(cx, |this, cx| this.sort_by(key, cx));
        }
    };
    let order = |desc: bool| {
        let panel = panel.clone();
        move |_: &mut Window, cx: &mut App| {
            let _ = panel.update(cx, |this, cx| this.set_query(this.query.clone().sorted(current, desc), cx));
        }
    };
    let keys = SortKey::ALL.iter().map(|key| MenuItem::new(format!("vault.sort.{}", key.key()), MenuText::keyed(format!("vault.sort.{}", key.key()), sort_label(*key))).checked(*key == current).on_click(by(*key)));
    let order = [(false, "menus.sort.ascending", LucideIcon::ArrowUp), (true, "menus.sort.descending", LucideIcon::ArrowDown)].map(|(desc, key, icon)| MenuItem::new(key, MenuText::key(key)).icon(icon).checked(desc == descending).on_click(order(desc)));
    MenuSpec::new().section(MenuSection::labelled("menus.sort.by").entries(keys)).section(MenuSection::labelled("menus.sort.order").entries(order))
}

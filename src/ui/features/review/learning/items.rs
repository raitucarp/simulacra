//! A compact chooser of review items, filtered by the search field: used to link items to
//! a goal and to place items at a palace locus.

use super::super::panel::{PanelFields, ReviewPanel};
use super::super::render::common::caption;
use super::super::text::{kind_label, tr};
use super::render::framed_input;
use crate::domain::review::{ItemId, ReviewItem};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use std::rc::Rc;

/// Items listed at most.
pub const CHOOSER_ROWS: usize = 8;
/// Lines of an item's prompt shown.
const PROMPT_LINES: usize = 2;

/// What a click on an item does.
pub type ItemAction = Rc<dyn Fn(ItemId, &mut Window, &mut App)>;

/// Whether `text` holds every word of `query`.
pub fn matches_words(text: &str, query: &str) -> bool {
    let haystack = text.to_lowercase();
    query.to_lowercase().split_whitespace().all(|word| haystack.contains(word))
}

impl ReviewPanel {
    /// The line an item is listed under (a map's title, else its prompt).
    pub(in crate::ui::features::review) fn item_line(&self, item: &ReviewItem) -> String {
        item.subject.map().map_or_else(|| self.data.prompt_text(item), |map| map.title.clone())
    }

    /// Live items matching the search, `skip` left out, at most [`CHOOSER_ROWS`].
    fn chooser_items(&self, skip: &[ItemId], cx: &App) -> Vec<ReviewItem> {
        let query = PanelFields::text(&self.learn_fields.place_search, cx);
        self.data.items.iter().filter(|item| !item.retired && !skip.contains(&item.id)).filter(|item| matches_words(&self.item_line(item), &query)).take(CHOOSER_ROWS).cloned().collect()
    }

    /// The search field over the chooser's rows.
    pub(in crate::ui::features::review) fn render_item_chooser(&self, id_prefix: &'static str, skip: &[ItemId], on_pick: ItemAction, cx: &App) -> Div {
        let theme = &self.theme;
        let items = self.chooser_items(skip, cx);
        let rows = items.iter().enumerate().map(|(index, item)| {
            let (id, pick) = (item.id.clone(), on_pick.clone());
            div()
                .id(ElementId::NamedInteger(id_prefix.into(), index as u64))
                .flex()
                .items_center()
                .gap(theme.inline_gap)
                .py(theme.spacing_3xs)
                .px(theme.spacing_xs)
                .rounded(theme.radius_sm)
                .cursor_pointer()
                .hover(|row| row.bg(theme.hover_bg))
                .on_click(move |_, window, cx| pick(id.clone(), window, cx))
                .child(Icon::new(LucideIcon::Plus).size(theme.icon_size_xs).text_color(theme.accent_brand))
                .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).line_clamp(PROMPT_LINES).child(self.item_line(item)))
                .child(div().flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(kind_label(item.subject.key())))
        });
        let empty = items.is_empty().then(|| caption(tr("learning.items.none"), theme));
        div().flex().flex_col().gap(theme.spacing_xs).child(framed_input(&self.learn_fields.place_search, theme)).children(empty).children(rows)
    }
}

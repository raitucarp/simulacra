//! N3: the garden view: beds of garden notes by maturity (evergreen, budding, seed), each
//! note with its stage's art, its text on one line, the thread it heads and the day it was
//! planted; a click opens it. A locked note shows only its placeholder.

use super::common::{empty_note, locked_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::post::garden::Maturity;
use crate::domain::views::garden::{garden, GardenBed, GardenEntry};
use crate::i18n::t_count;
use crate::ui::features::day_filter::format_day;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::*;
use rust_i18n::t;

/// The art of a garden stage, `size` wide.
pub fn maturity_art(maturity: Maturity, size: Pixels) -> Illustration {
    Illustration::asset(maturity.art_path(), IllustrationKind::Symbol).width(size)
}

/// The translated name of a garden stage.
pub fn maturity_label(maturity: Maturity) -> String {
    let key = format!("posts.garden.stage.{}", maturity.key());
    t!(&key).to_string()
}

pub(super) fn render_garden(input: &ViewInput<'_>) -> AnyElement {
    let beds = garden(input.posts.iter().copied());
    if beds.is_empty() {
        return empty_note("views.garden.empty", input.theme);
    }
    scroll_column(input, "garden").children(beds.iter().map(|bed| render_bed(bed, input))).into_any_element()
}

fn render_bed(bed: &GardenBed, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let title = format!("{} · {}", maturity_label(bed.maturity), bed.notes.len());
    let head = div().flex().items_center().gap(theme.inline_gap).child(maturity_art(bed.maturity, theme.symbol_badge_size)).child(section_heading(title, theme));
    div().flex().flex_col().gap(theme.spacing_3xs).child(head).children(bed.notes.iter().map(|note| render_note(note, bed.maturity, input)))
}

fn render_note(note: &GardenEntry, maturity: Maturity, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let text = match note.locked {
        true => locked_note(theme).into_any_element(),
        false => div().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(note.text.clone()).into_any_element(),
    };
    let planted = t!("views.garden.planted", date = format_day(crate::time::local_date(note.planted))).to_string();
    let thread = (note.replies > 0).then(|| t_count("views.garden.replies", note.replies));
    let meta = div().flex().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(planted).children(thread);
    let column = div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_3xs).child(text).child(meta);
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("garden-{}", note.post_id)))
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(input.opener(&note.post_id))
        .child(div().flex_none().child(maturity_art(maturity, theme.symbol_outline_size)))
        .child(column)
}

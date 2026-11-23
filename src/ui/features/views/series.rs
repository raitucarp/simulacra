//! N10: the series view: each series among the posts, with its progress line and its parts
//! in order ("#12 …"); a click opens a part. A locked part shows only its placeholder.

use super::common::{empty_note, locked_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::post::series::progress;
use crate::domain::views::series::{series_groups, SeriesGroup, SeriesPart};
use crate::i18n::t_count;
use gpui_kit::*;
use rust_i18n::t;

pub(super) fn render_series(input: &ViewInput<'_>) -> AnyElement {
    let groups = series_groups(input.posts.iter().copied());
    if groups.is_empty() {
        return empty_note("views.series.empty", input.theme);
    }
    scroll_column(input, "series").children(groups.iter().map(|group| render_group(group, input))).into_any_element()
}

fn render_group(group: &SeriesGroup, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let count = match group.goal {
        Some(goal) => t!("views.series.progress", posted = group.parts.len(), goal = goal).to_string(),
        None => t_count("views.series.parts", group.parts.len()),
    };
    let share = progress(group.parts.len(), group.goal);
    let line = div().w_full().h(theme.journal_progress_height).rounded(theme.radius_full).bg(theme.border_subtle).overflow_hidden().child(div().h_full().w(relative(share)).rounded(theme.radius_full).bg(theme.accent_brand));
    let head = div().flex().flex_col().gap(theme.spacing_3xs).child(section_heading(format!("{} · {count}", group.title), theme)).child(line);
    div().flex().flex_col().gap(theme.spacing_3xs).child(head).children(group.parts.iter().map(|part| render_part(group, part, input)))
}

fn render_part(group: &SeriesGroup, part: &SeriesPart, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let text = match part.locked {
        true => locked_note(theme).into_any_element(),
        false => div().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(part.text.clone()).into_any_element(),
    };
    let number = div().flex_none().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(format!("#{}", part.number));
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("series-{}-{}", group.id, part.post_id)))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(input.opener(&part.post_id))
        .child(number)
        .child(text)
}

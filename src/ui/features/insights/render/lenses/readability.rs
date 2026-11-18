//! The Readability view: key numbers (median ease, median grade level, share of hard posts,
//! posts scored), the bands per author, the median ease per period, the hardest and easiest
//! posts (a row opens the post), and how each score was measured, with the formulas' sources.

use super::super::common::{card, card_title, pair, tile_columns, tile_row, tr};
use super::super::Room;
use super::{explained_tile, posts_label, short_day};
use crate::domain::insights::lenses::readability::ReadabilityReport;
use crate::domain::insights::lenses::LensReport;
use crate::domain::readability::{Band, Readability};
use crate::domain::PostId;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::features::day_filter::format_day;
use crate::ui::features::insights::chart::columns::{Column, ColumnChart};
use crate::ui::features::insights::chart::legend::{legend, LegendEntry};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::percent;
use crate::ui::features::insights::events::InsightsEvent;
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::features::insights::readability_style::{band_colour, band_label, ease_text, formula_label, formula_source};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

impl InsightsPanel {
    pub(in super::super) fn render_readability(&self, report: &Rc<LensReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let data = &report.readability;
        let Some(median) = data.median_ease else { return self.view_empty("insight.readability.empty_title", "insight.readability.empty_hint") };
        let band = Band::of_ease(median);
        let grade = data.median_grade.map_or_else(|| tr("insight.none"), |g| format!("{:.1}", g.max(0.0)));
        let tiles = vec![
            explained_tile(ease_text(median), tr("insight.readability.median_label"), t!("insight.readability.median_meaning", band = band_label(band)).to_string(), band_colour(band, theme), theme),
            explained_tile(grade, tr("insight.readability.grade_label"), tr("insight.readability.grade_meaning"), theme.info, theme),
            explained_tile(percent(data.hard_share), tr("insight.readability.hard_label"), tr("insight.readability.hard_meaning"), theme.warning, theme),
            explained_tile(data.scored.len().to_string(), tr("insight.readability.scored_label"), t!("insight.readability.scored_meaning", posts = posts_label(report.posts.len())).to_string(), theme.accent_brand, theme),
        ];
        let row = tile_row(tile_columns(room, tiles.len()), theme).children(tiles);
        let extremes = pair(room, self.extremes_card("insight-hardest", tr("insight.readability.hardest"), &data.hardest, report, cx), self.extremes_card("insight-easiest", tr("insight.readability.easiest"), &data.easiest, report, cx), theme);
        div().w_full().flex().flex_col().gap(theme.section_gap).child(row).child(self.bands_by_author_card(report)).child(pair(room, self.ease_trend_card(data), self.method_card(data), theme)).child(extremes)
    }

    fn bands_by_author_card(&self, report: &LensReport) -> Div {
        let theme = &self.theme;
        let name = |a: usize| report.authors.get(a).map(|f| f.name.clone()).unwrap_or_default();
        let columns = report.readability.authors.iter().map(|row| {
            let parts: Vec<String> = Band::ALL.iter().filter(|b| row.bands[b.index()] > 0).map(|b| format!("{} {}", band_label(*b), row.bands[b.index()])).collect();
            Column { segments: Band::ALL.iter().map(|b| (row.bands[b.index()] as f64, band_colour(*b, theme))).collect(), tip: format!("{} · {}: {}", name(row.author), ease_text(row.median_ease), parts.join(", ")) }
        });
        let labels = report.readability.authors.iter().map(|row| name(row.author)).collect();
        let chart = ColumnChart { id: "insight-bands-by-author".into(), columns: columns.collect(), labels, height: theme.insights_chart_height, on_click: None, theme: theme.clone() };
        let entries: Vec<LegendEntry> = Band::ALL.iter().map(|b| LegendEntry { colour: band_colour(*b, theme), label: band_label(*b), value: None, hidden: false }).collect();
        card(theme).child(card_title(LucideIcon::Users, tr("insight.readability.by_author"), None, theme)).child(chart).child(legend("insight-band-legend", &entries, None, theme)).child(summary_line(tr("insight.readability.by_author_hint"), theme))
    }

    fn ease_trend_card(&self, data: &ReadabilityReport) -> Div {
        let theme = &self.theme;
        let periods = &data.periods;
        let columns = data.trend.iter().map(|point| {
            let span = format!("{} – {}", format_day(periods.start_of(point.period)), format_day(periods.end_of(point.period)));
            let colour = point.median_ease.map_or(theme.border_subtle, |ease| band_colour(Band::of_ease(ease), theme));
            Column { segments: vec![(point.median_ease.unwrap_or_default(), colour)], tip: t!("insight.readability.trend_tip", span = span, ease = point.median_ease.map_or_else(|| tr("insight.none"), ease_text), posts = posts_label(point.posts)).to_string() }
        });
        let labels = data.trend.iter().map(|p| short_day(periods.start_of(p.period))).collect();
        let chart = ColumnChart { id: "insight-ease-trend".into(), columns: columns.collect(), labels, height: theme.insights_chart_height, on_click: None, theme: theme.clone() };
        card(theme).child(card_title(LucideIcon::ChartColumn, tr("insight.readability.trend"), None, theme)).child(chart).child(summary_line(tr("insight.readability.trend_hint"), theme))
    }

    /// The formulas used, how many posts each scored, and where each was published.
    fn method_card(&self, data: &ReadabilityReport) -> Div {
        let theme = &self.theme;
        let lines = data.formulas.iter().map(|(formula, count)| {
            let text = t!("insight.readability.method_line", formula = formula_label(*formula), count = posts_label(*count), source = formula_source(*formula)).to_string();
            summary_line(text, theme)
        });
        card(theme).child(card_title(LucideIcon::FlaskConical, tr("insight.readability.method"), None, theme)).children(lines).child(summary_line(tr("insight.readability.method_hint"), theme))
    }

    fn extremes_card(&self, id: &'static str, title: String, picks: &[usize], report: &Rc<LensReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows = picks.iter().enumerate().filter_map(|(row, &i)| {
            let scored = &report.readability.scored[i];
            report.posts.get(scored.post).map(|post| self.extreme_row(id, row, post.id.clone(), &post.excerpt, &scored.score, cx))
        });
        card(theme).child(card_title(LucideIcon::FileText, title, None, theme)).child(div().w_full().flex().flex_col().gap(theme.spacing_3xs).children(rows))
    }

    /// One post: its excerpt and a badge with its ease and band; opens the post.
    fn extreme_row(&self, id: &'static str, row: usize, post: PostId, excerpt: &str, score: &Readability, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let entity = cx.entity().downgrade();
        let open: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(move |_, cx| {
            let post = post.clone();
            let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::OpenPost(post)));
        });
        let click = Rc::clone(&open);
        let colour = band_colour(score.band, theme);
        let badge = div().flex_none().px(theme.spacing_xs).rounded(theme.radius_full).border(theme.border_width_hairline).border_color(colour).text_size(theme.font_size_2xs).text_color(theme.text_secondary).child(format!("{} · {}", ease_text(score.ease), band_label(score.band)));
        div()
            .id(ElementId::NamedInteger(id.into(), row as u64))
            .flex()
            .items_start()
            .gap(theme.control_gap)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .cursor_pointer()
            .hover(|s| s.bg(theme.hover_bg))
            .on_click(move |_, window, cx| click(window, cx))
            .keyboard_stop(theme, open)
            .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(excerpt.to_string()))
            .child(badge)
    }
}

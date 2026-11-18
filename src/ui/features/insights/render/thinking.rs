//! sb E1: the Calibration and Tensions views, laid out with the page's cards, tiles and chart
//! kit from the thinking tools' snapshot (`InsightsPanel::thinking`, set by the shell).

use super::super::events::InsightsEvent;
use super::super::panel::InsightsPanel;
use super::super::view::InsightsView;
use super::common::{card, card_title, option_chip, pair, tile, tile_columns, tile_row, tr};
use super::Room;
use crate::ui::components::{art_state, ArtState};
use crate::ui::features::insights::chart::columns::ColumnChart;
use crate::ui::features::insights::chart::ranked::ranked_list;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::thinking::insights::calibration::{histogram, key_figures, murphy_rows, prediction_list};
use crate::ui::features::thinking::insights::reliability::reliability_diagram;
use crate::ui::features::thinking::insights::tensions::tension_card;
use crate::ui::features::thinking::insights::{RequestAction, TensionMode, ThinkingRequest};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration, NoticeKey};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

impl InsightsPanel {
    /// A request callback that hands `ThinkingRequest`s to the shell.
    fn thinking_requests(&self, cx: &mut Context<Self>) -> RequestAction {
        let entity = cx.entity().downgrade();
        Rc::new(move |request, _, cx| {
            let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::Thinking(request)));
        })
    }

    pub(super) fn render_thinking(&self, room: Room, cx: &mut Context<Self>) -> AnyElement {
        match self.state.view {
            InsightsView::Tensions => self.render_tensions(room, cx).into_any_element(),
            _ => self.render_calibration(room, cx).into_any_element(),
        }
    }

    fn render_calibration(&self, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let data = &self.thinking.calibration;
        let on_request = self.thinking_requests(cx);
        let due_chip = option_chip("calibration-due-only".into(), tr("thinking.calibration.due_only"), data.due_only, theme, { let r = on_request.clone(); Rc::new(move |w, cx| r(ThinkingRequest::ToggleDueOnly, w, cx)) });
        let list = card(theme).child(card_title(LucideIcon::CalendarClock, tr("thinking.calibration.open_title"), Some(due_chip.into_any_element()), theme)).child(prediction_list(data, &on_request, theme));
        let Some(summary) = data.summary.as_ref().filter(|s| s.resolved > 0) else {
            let art = Illustration::empty(EmptyKey::NoResults).width(theme.art_empty_width);
            let empty = card(theme).items_center().child(art_state(ArtState::titled(art, tr("thinking.calibration.empty_title"), tr("thinking.calibration.empty_hint")), theme));
            return div().w_full().flex().flex_col().gap(theme.section_gap).child(empty).child(list);
        };
        let figures = key_figures(summary, theme);
        let tiles = tile_row(tile_columns(room, figures.len()), theme).children(figures.into_iter().map(|(value, label, colour)| tile(value, label, colour, theme)));
        let diagram = card(theme).child(card_title(LucideIcon::ChartScatter, tr("thinking.calibration.curve"), None, theme)).child(div().w_full().flex().justify_center().child(reliability_diagram(summary.bins.clone(), theme))).child(summary_line(tr("thinking.calibration.curve_hint"), theme));
        let (columns, labels) = histogram(summary, theme);
        let chart = ColumnChart { id: "calibration-histogram".into(), columns, labels, height: theme.insights_chart_height, on_click: None, theme: theme.clone() };
        let murphy = ranked_list("calibration-murphy", &murphy_rows(summary, theme), None, theme);
        let spread = card(theme).child(card_title(LucideIcon::ChartColumn, tr("thinking.calibration.histogram"), None, theme)).child(chart).child(card_title(LucideIcon::Scale, tr("thinking.calibration.murphy_title"), None, theme)).child(murphy).child(summary_line(tr("thinking.calibration.murphy_hint"), theme));
        div().w_full().flex().flex_col().gap(theme.section_gap).child(tiles).child(pair(room, diagram, spread, theme)).child(list)
    }

    fn render_tensions(&self, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let data = &self.thinking.tensions;
        let on_request = self.thinking_requests(cx);
        let scan = Button::new("tensions-scan").ghost().xsmall().icon(Icon::new(LucideIcon::RefreshCw)).label(tr("thinking.tensions.scan")).on_click({ let r = on_request.clone(); move |_, w, cx| r(ThinkingRequest::ScanTensions, w, cx) });
        let focus = data.focus.clone().map(|words| {
            let r = on_request.clone();
            option_chip("tensions-focus".into(), t!("thinking.tensions.focus", words = words).to_string(), true, theme, Rc::new(move |w, cx| r(ThinkingRequest::ClearFocus, w, cx)))
        });
        let title = if data.running { tr("thinking.tensions.running") } else { t!("thinking.tensions.count", count = data.rows.len()).to_string() };
        let header = card(theme).child(card_title(LucideIcon::Zap, title, Some(scan.into_any_element()), theme)).children(focus).children(self.tension_notice(data.mode, data.problem.clone()));
        let cards = data.rows.iter().enumerate().map(|(i, row)| card(theme).child(tension_card(i, row, room == Room::Narrow, &on_request, theme)));
        let empty = (data.rows.is_empty() && data.scanned && !data.running).then(|| {
            let art = Illustration::empty(EmptyKey::NoResults).width(theme.art_empty_width);
            card(theme).items_center().child(art_state(ArtState::titled(art, tr("thinking.tensions.empty_title"), tr("thinking.tensions.empty_hint")), theme))
        });
        div().w_full().flex().flex_col().gap(theme.section_gap).child(header).children(empty).children(cards)
    }

    /// Why only word-based candidates are shown, or why the last check failed.
    fn tension_notice(&self, mode: TensionMode, problem: Option<String>) -> Option<Div> {
        let theme = &self.theme;
        let (key, hint) = match (mode, problem) {
            (_, Some(problem)) => (NoticeKey::Error, problem),
            (TensionMode::LexicalOnly, None) => (NoticeKey::NoKey, tr("thinking.tensions.lexical_only")),
            (TensionMode::NoMeaning, None) => (NoticeKey::Offline, tr("thinking.tensions.no_meaning")),
            (TensionMode::Full, None) => return None,
        };
        let state = ArtState::hint(Illustration::notice(key).width(theme.art_compact_width), hint);
        Some(div().w_full().flex().justify_center().child(art_state(state, theme)))
    }
}

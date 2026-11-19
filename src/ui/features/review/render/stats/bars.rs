//! A column chart sized to its container: as many bars as values, each as wide as the
//! measured width allows (up to the token maximum), with a tooltip per bar, axis labels
//! under the first, middle and last bars, and an optional target line.

use crate::theme::ThemeTokens;
use crate::ui::components::measured_width;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const BAR_ID_PREFIX: &str = "review-bar-";

/// One column chart.
#[derive(IntoElement)]
pub struct BarChart {
    pub id: &'static str,
    /// Bar heights; `None` draws an empty slot (no data).
    pub values: Vec<Option<f64>>,
    /// The value of a full-height bar.
    pub max: f64,
    pub color: Rgba,
    /// Tooltip of each bar.
    pub tips: Vec<String>,
    /// Labels under the first, middle and last bar.
    pub axis: [String; 3],
    /// A horizontal guide at this value (the retention target).
    pub target: Option<f64>,
    pub theme: ThemeTokens,
}

impl BarChart {
    fn bar(&self, index: usize, value: Option<f64>, width: Pixels) -> Stateful<Div> {
        let theme = &self.theme;
        let share = value.map_or(0.0, |v| (v / self.max.max(f64::MIN_POSITIVE)).clamp(0.0, 1.0)) as f32;
        let tip = self.tips.get(index).cloned().unwrap_or_default();
        let fill = div().w_full().min_h(theme.review_bar_min_height).h(relative(share)).rounded(theme.radius_xs).bg(if value.is_some() { self.color } else { theme.border_subtle });
        div()
            .id(ElementId::Name(format!("{BAR_ID_PREFIX}{}-{index}", self.id).into()))
            .w(width)
            .h_full()
            .flex()
            .flex_col()
            .justify_end()
            .hover(|bar| bar.opacity(theme.graph_dim_opacity))
            .tooltip(move |window, cx| Tooltip::new(tip.clone()).build(window, cx))
            .child(fill)
    }

    fn axis_row(&self) -> Div {
        let theme = &self.theme;
        let label = |text: String| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text);
        let [first, middle, last] = self.axis.clone();
        div().w_full().flex().justify_between().child(label(first)).child(label(middle)).child(label(last))
    }
}

impl RenderOnce for BarChart {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (measured, probe) = measured_width(self.id, window, cx);
        let theme = self.theme.clone();
        let count = self.values.len().max(1) as f32;
        let width = measured.unwrap_or(theme.review_card_max_width);
        let gap = theme.spacing_3xs;
        let bar_width = ((width - gap * (count - 1.0)) / count).min(theme.review_bar_max_width).max(theme.border_width_hairline);
        let bars: Vec<Stateful<Div>> = self.values.iter().enumerate().map(|(i, value)| self.bar(i, *value, bar_width)).collect();
        let target = self.target.map(|value| (value / self.max.max(f64::MIN_POSITIVE)).clamp(0.0, 1.0) as f32);
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(theme.review_chart_height)
                    .flex()
                    .items_end()
                    .justify_between()
                    .child(probe)
                    .children(bars)
                    .when_some(target, |chart, share| {
                        chart.child(div().absolute().left_0().right_0().bottom(relative(share)).h(theme.border_width_hairline).bg(theme.accent_dialectic))
                    }),
            )
            .child(self.axis_row())
    }
}

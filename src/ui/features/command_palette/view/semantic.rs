//! Semantic mode of the palette (sb B3): asks the shell to search by meaning, shows a
//! status line (prompt, searching, nothing close, or the word-match note) and scored rows.

use super::rows::render_group_heading;
use super::{CommandPalette, PaletteEvent};
use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::model::{ResultGroup, SemanticPhase, SemanticSource};
use crate::ui::features::semantic::render_scored_row;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

impl CommandPalette {
    /// The semantic groups for the current text, asking the shell when the text changed.
    pub(super) fn semantic_groups(&mut self, cx: &mut Context<Self>) -> Vec<ResultGroup> {
        let text = self.query.text.clone();
        if text.is_empty() {
            self.semantic = SemanticPhase::Prompt;
            return Vec::new();
        }
        if self.semantic.query() != Some(text.as_str()) {
            self.semantic = SemanticPhase::Searching { query: text.clone() };
            cx.emit(PaletteEvent::SemanticQuery(text));
        }
        self.semantic.groups()
    }

    /// Shows `phase` if it answers the current query (late answers to older text are dropped).
    pub fn show_semantic_results(&mut self, phase: SemanticPhase, cx: &mut Context<Self>) {
        if self.query.mode != crate::ui::features::command_palette::PaletteMode::Semantic || phase.query() != Some(self.query.text.as_str()) {
            return;
        }
        self.semantic = phase;
        self.groups = self.semantic.groups();
        self.selected = 0;
        self.scroll.scroll_to_item(0);
        cx.notify();
    }

    /// The text currently searched by meaning, if any.
    pub fn semantic_query(&self) -> Option<String> {
        (self.query.mode == crate::ui::features::command_palette::PaletteMode::Semantic && !self.query.text.is_empty()).then(|| self.query.text.clone())
    }

    /// Status line, heading and scored rows.
    pub(super) fn render_semantic_results(&self, theme: &ThemeTokens, cx: &mut Context<Self>) -> AnyElement {
        let mut children: Vec<AnyElement> = self.semantic_status().map(|(icon, text)| render_status(icon, text, theme).into_any_element()).into_iter().collect();
        let rows = self.semantic.rows();
        for group in &self.groups {
            children.push(render_group_heading(group.group, theme).into_any_element());
            for ranked in &group.items {
                let (index, row) = (ranked.index, &rows[ranked.index]);
                let element = render_scored_row(index, row.item.icon, &row.item.title, &row.item.detail, &row.excerpt, row.score, index == self.selected, theme);
                children.push(element.on_click(cx.listener(move |this, _, _, cx| this.run(index, cx))).into_any_element());
            }
        }
        self.results_list(theme, children)
    }

    /// The line above the rows, when there is something to say.
    fn semantic_status(&self) -> Option<(LucideIcon, String)> {
        match &self.semantic {
            SemanticPhase::Prompt => Some((LucideIcon::Sparkles, t!("palette.semantic.prompt").to_string())),
            SemanticPhase::Searching { .. } => Some((LucideIcon::RefreshCw, t!("palette.semantic.searching").to_string())),
            SemanticPhase::Ready { rows, query, .. } if rows.is_empty() => Some((LucideIcon::Search, t!("palette.semantic.empty", query = query.as_str()).to_string())),
            SemanticPhase::Ready { source: SemanticSource::Words { note }, .. } => Some((LucideIcon::Info, note.clone())),
            SemanticPhase::Ready { .. } => None,
        }
    }
}

fn render_status(icon: LucideIcon, text: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(Icon::new(icon).size(theme.icon_size_xs))
        .child(div().flex_1().min_w_0().child(text))
}

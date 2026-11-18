//! What the analysis found, as a short list: claims nothing supports, support that runs in
//! a circle, objections nobody has answered. Each entry opens its node in the editor.

use super::super::labels::tr;
use super::{ArgumentPanel, ArgumentThread};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const FINDING_ID: &str = "argument-finding";

/// One finding: its icon, colour, text and the node it opens.
struct Entry {
    icon: LucideIcon,
    colour: Rgba,
    text: String,
    node: usize,
}

impl ArgumentPanel {
    fn entries(&self, thread: &ArgumentThread) -> Vec<Entry> {
        let theme = &self.theme;
        let text = |n: usize| thread.map.nodes.get(n).map(|node| node.text.clone()).unwrap_or_default();
        let unsupported = self.report.unsupported.iter().map(|n| Entry { icon: LucideIcon::CircleAlert, colour: theme.warning, text: t!("thinking.argument.finding.unsupported", text = text(*n)).to_string(), node: *n });
        let circular = self.report.circular.iter().map(|cycle| {
            let names: Vec<String> = cycle.iter().map(|n| text(*n)).collect();
            Entry { icon: LucideIcon::RefreshCw, colour: theme.accent_danger, text: t!("thinking.argument.finding.circular", texts = names.join(" → ")).to_string(), node: cycle[0] }
        });
        let unanswered = self.report.unanswered.iter().map(|n| Entry { icon: LucideIcon::MessageSquareReply, colour: theme.accent_dialectic, text: t!("thinking.argument.finding.unanswered", text = text(*n)).to_string(), node: *n });
        unsupported.chain(circular).chain(unanswered).collect()
    }

    /// The findings card, or `None` when the argument shows none of these problems.
    pub(super) fn render_findings(&self, thread: &ArgumentThread, cx: &mut Context<Self>) -> Option<Div> {
        let theme = &self.theme;
        let entries = self.entries(thread);
        if entries.is_empty() {
            return None;
        }
        let rows = entries.into_iter().enumerate().map(|(i, entry)| {
            let node = entry.node;
            div()
                .id(ElementId::NamedInteger(FINDING_ID.into(), i as u64))
                .w_full()
                .flex()
                .items_start()
                .gap(theme.inline_gap)
                .px(theme.spacing_xs)
                .py(theme.spacing_3xs)
                .rounded(theme.radius_sm)
                .cursor_pointer()
                .hover(|row| row.bg(theme.hover_bg))
                .on_click(cx.listener(move |panel, _, window, cx| panel.pick_node(node, window, cx)))
                .child(Icon::new(entry.icon).size(theme.icon_size_xs).text_color(entry.colour))
                .child(div().flex_1().min_w_0().text_size(theme.font_size_2xs).line_height(relative(theme.line_height_body)).text_color(theme.text_secondary).child(entry.text))
        });
        let title = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(tr("thinking.argument.findings"));
        Some(div().w_full().flex().flex_col().gap(theme.spacing_3xs).child(title).children(rows))
    }
}

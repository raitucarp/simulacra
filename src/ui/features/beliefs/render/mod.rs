//! The Beliefs tab: the header (what this is, the update switch and today's count, "Read the
//! open thread") above the beliefs, most salient first, or an empty state.

mod edit;
mod history;
mod row;
pub(super) mod text;

use super::events::BeliefsEvent;
use super::BeliefsPanel;
use crate::ui::components::{art_state, choice_chip, ArtState};
use crate::ui::features::arena::kit::{block_title, caption, control_row, small_button, surface};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::base::Disableable;
use gpui_kit::*;
use rust_i18n::t;
use text::tr;

const ROOT_ID: &str = "beliefs-root";
const SWITCH_ID: &str = "beliefs-updates-switch";
const READ_ID: &str = "beliefs-read-thread";

impl BeliefsPanel {
    fn render_header(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let data = &self.data;
        let Some(agent) = data.agent.clone() else { return div() };
        let on = data.settings.updates_enabled;
        let (switch_agent, read_agent, switch_entity, read_entity) = (agent.clone(), agent, cx.entity().downgrade(), cx.entity().downgrade());
        let switch = choice_chip(SWITCH_ID, &tr(if on { "beliefs.updates_on" } else { "beliefs.updates_off" }), on, theme, move |_, cx| {
            let _ = switch_entity.update(cx, |_, cx| cx.emit(BeliefsEvent::SetUpdates(switch_agent.clone(), !on)));
        });
        let can_read = data.thread_open && !data.updating && on && data.used_today < data.daily_cap;
        let read = small_button(READ_ID, LucideIcon::BookOpen, tr("beliefs.read_thread")).disabled(!can_read).on_click(move |_, _, cx| {
            let _ = read_entity.update(cx, |_, cx| cx.emit(BeliefsEvent::ReadOpenThread(read_agent.clone())));
        });
        let quota = t!("beliefs.quota", used = data.used_today, cap = data.daily_cap).to_string();
        let status = if data.updating { tr("beliefs.updating") } else { quota };
        surface(theme)
            .child(block_title(LucideIcon::Brain, theme.accent_agent, t!("beliefs.title", name = data.agent_name).to_string(), theme))
            .child(caption(tr("beliefs.hint"), theme))
            .child(control_row(theme).child(switch).child(read))
            .child(caption(status, theme))
            .children(data.notice.clone().map(|notice| caption(notice, theme).text_color(theme.warning)))
    }

    fn render_empty(&self) -> Div {
        let theme = &self.theme;
        let state = ArtState::titled(Illustration::empty(EmptyKey::NoResults).width(theme.art_compact_width), tr("beliefs.empty_title"), tr("beliefs.empty_hint"));
        surface(theme).items_center().child(art_state(state, theme))
    }
}

impl Render for BeliefsPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let rows: Vec<Div> = self.data.rows.iter().enumerate().map(|(index, row)| self.render_row(index, row, cx)).collect();
        let body = if rows.is_empty() { self.render_empty() } else { div().w_full().flex().flex_col().gap(theme.stack_gap).children(rows) };
        div().id(ROOT_ID).w_full().flex().flex_col().gap(theme.section_gap).child(self.render_header(cx)).child(body)
    }
}

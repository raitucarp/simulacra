//! N16: the lens fan in the shell. The sheet picks the lenses; sending gives each lens to the
//! best-fitting agent that may read the post (N8), one lens per agent, records the fan on the
//! post after the spend check, and starts each agent's reply through its lens on the usual
//! generation path (its engine, the queue and the spend rules). A locked post is never fanned.

use super::super::view::AppView;
use crate::agent::JobKind;
use crate::domain::post::lens::assign;
use crate::domain::spend::SpendEstimate;
use crate::state::store::agent_knowledge;
use crate::ui::features::data::{lens_name, tr, LensDialog, LensEvent};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one lens fan event.
    pub(in crate::ui::features::shell) fn on_lens_event(&mut self, event: LensEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            LensEvent::Open(id) => {
                let Some(post) = self.state.get_post(&id).filter(|post| !post.is_locked()) else { return };
                let room = self.state.room_for(post).len();
                self.data.ideas.thoughts.lens = Some(LensDialog::new(id, room));
            }
            LensEvent::Toggle(lens) => {
                if let Some(dialog) = self.data.ideas.thoughts.lens.as_mut() {
                    dialog.problem = None;
                    dialog.toggle(lens);
                }
            }
            LensEvent::Send => self.send_lens_fan(window, cx),
            LensEvent::Close => self.data.ideas.thoughts.lens = None,
        }
    }

    fn lens_problem(&mut self, key: &str) {
        if let Some(dialog) = self.data.ideas.thoughts.lens.as_mut() {
            dialog.problem = Some(tr(key));
        }
    }

    /// Fans the sheet's post out through its lenses, after the spend check.
    fn send_lens_fan(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.lens.clone() else { return };
        let Some(post) = self.state.get_post(&dialog.post_id).filter(|post| !post.is_locked()) else { return }; // X2
        if dialog.chosen.is_empty() {
            return self.lens_problem("posts.lens.pick_one");
        }
        let room: Vec<_> = self.state.room_for(post).into_iter().map(|agent| (agent.id.clone(), agent_knowledge(agent))).collect();
        let (fan, parent, network, id) = (assign(&dialog.chosen, &room), self.state.authored(post), post.network_id.clone(), dialog.post_id.clone());
        if fan.is_empty() {
            return self.lens_problem("posts.lens.nobody");
        }
        if self.confirm_spend(SpendEstimate::requests(fan.len() as u32), Box::new(|this, window, cx| this.send_lens_fan(window, cx)), cx) {
            return;
        }
        if self.state.fan_out(&id, fan.clone(), jiff::Timestamp::now()).is_err() {
            return self.lens_problem("posts.lens.failed");
        }
        self.data.ideas.thoughts.lens = None;
        let mut names = Vec::new();
        for entry in fan {
            let Some(agent) = self.state.user_profiles.iter().find(|p| p.id == entry.agent).cloned() else { continue };
            names.push(format!("{} ({})", agent.name, lens_name(entry.lens)));
            if self.engine_allowed(&agent) && self.scheduler.queue.start_now(&agent.id) {
                self.spawn_generation(agent, JobKind::Lens { parent: parent.clone(), lens: entry.lens }, network.clone(), window, cx);
            }
        }
        self.feed.selected_thread_post_id = Some(id);
        self.notify_data(t!("posts.lens.asked", names = names.join(", ")).to_string(), window, cx);
    }
}

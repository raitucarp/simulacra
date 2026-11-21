//! The clap button in the shell (R7): a press starts timing a hold, a burst claps on a timer
//! while the button stays down, and the release claps once, takes one back (Alt) or ends
//! the burst.

use super::super::reactions_host::ClapHold;
use super::super::view::AppView;
use crate::domain::reaction::gesture::{owed, release, ClapRelease, BURST_INTERVAL};
use crate::domain::PostId;
use gpui_kit::*;
use std::time::Instant;

impl AppView {
    /// The button went down on `post` (Alt held: `alt`): times the hold on a ticking task.
    pub(super) fn press_clap(&mut self, post: PostId, alt: bool, window: &mut Window, cx: &mut Context<Self>) {
        let task = cx.spawn_in(window, async move |entity, cx| loop {
            cx.background_executor().timer(BURST_INTERVAL).await;
            if entity.update(cx, |this, cx| this.tick_clap_burst(cx)).is_err() {
                break;
            }
        });
        self.reactions.hold = Some(ClapHold { post, alt, started: Instant::now(), given: 0, _task: task });
    }

    /// Claps what the held button owes since the last tick.
    fn tick_clap_burst(&mut self, cx: &mut Context<Self>) {
        let Some(hold) = self.reactions.hold.as_mut() else { return };
        let due = if hold.alt { 0 } else { owed(hold.started.elapsed(), hold.given) };
        if due == 0 {
            return;
        }
        hold.given += due;
        let post = hold.post.clone();
        if self.state.clap(&post, due).is_some() {
            cx.notify();
        }
    }

    /// The button came up (or the pointer left it while down): finishes the gesture.
    pub(super) fn release_clap(&mut self, post: PostId, cx: &mut Context<Self>) {
        if self.reactions.hold.as_ref().is_none_or(|hold| hold.post != post) {
            return; // another card's button, or no press
        }
        let Some(hold) = self.reactions.hold.take() else { return };
        let held = hold.started.elapsed();
        match release(held, hold.alt) {
            ClapRelease::Clap => self.state.clap(&post, 1),
            ClapRelease::Unclap => self.state.unclap(&post, 1),
            ClapRelease::Done => self.state.clap(&post, owed(held, hold.given)),
        };
        cx.notify();
    }
}

//! N13: "Still true?" in the shell: confirming a claim (it comes back after a longer wait).

use super::super::view::AppView;
use crate::domain::post::still_true::wait_days;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::CheckEvent;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one "Still true?" event.
    pub(in crate::ui::features::shell) fn on_check_event(&mut self, event: CheckEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            CheckEvent::Confirm(id) => match self.state.confirm_still_true(&id, jiff::Timestamp::now()) {
                Ok(check) => self.notify_data(t!("posts.still_true.confirmed", days = wait_days(check.streak)).to_string(), window, cx),
                Err(err) => self.notify_data(publish_error_message(&err), window, cx),
            },
        }
    }
}

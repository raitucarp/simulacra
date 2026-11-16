//! "Create network" dialog: name a new thought-space, persist it and switch to it.

use super::super::view::AppView;
use crate::state::{NetworkNameError, MAX_NETWORK_NAME_CHARS};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::cell::Cell;

/// Notification id so repeated validation errors replace each other.
struct CreateNetworkNotice;

/// Localized explanation of a rejected network name.
fn network_name_message(err: &NetworkNameError) -> String {
    match err {
        NetworkNameError::Empty => t!("workspace.error_empty"),
        NetworkNameError::TooLong => t!("workspace.error_too_long", max = MAX_NETWORK_NAME_CHARS),
        NetworkNameError::Duplicate(name) => t!("workspace.error_duplicate", name = name),
        NetworkNameError::Storage(reason) => t!("workspace.error_storage", reason = reason),
    }
    .to_string()
}

impl AppView {
    /// Closes the workspace popover and asks for the new network's name.
    pub(crate) fn open_create_network(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_workspace_open = false;
        let placeholder = t!("workspace.create_placeholder").to_string();
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let view = cx.entity().downgrade();
        let needs_focus = Cell::new(true);
        window.open_alert_dialog(cx, move |alert, window, cx| {
            if needs_focus.replace(false) {
                input.update(cx, |state, cx| state.focus(window, cx));
            }
            let (input, view) = (input.clone(), view.clone());
            alert
                .confirm()
                .title(t!("workspace.create_title").to_string())
                .description(t!("workspace.create_desc").to_string())
                .child(Input::new(&input))
                .ok_text(t!("workspace.create_button").to_string())
                .cancel_text(t!("composer.cancel").to_string())
                .on_ok(move |_, window, cx| {
                    let name = input.read(cx).value().to_string();
                    view.update(cx, |this, cx| this.create_network_named(&name, window, cx))
                        .unwrap_or(true)
                })
        });
        cx.notify();
    }

    /// Creates and switches to the network; `false` keeps the dialog open after an error toast.
    fn create_network_named(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) -> bool {
        match self.state.create_network(name) {
            Ok(id) => {
                let note = t!("workspace.created", name = name.trim()).to_string();
                window.push_notification(Notification::success(note).id::<CreateNetworkNotice>(), cx);
                self.select_network(id, window, cx);
                true
            }
            Err(err) => {
                let note = Notification::warning(network_name_message(&err));
                window.push_notification(note.id::<CreateNetworkNotice>(), cx);
                false
            }
        }
    }
}

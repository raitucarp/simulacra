//! "Create network" dialog: name a new thought-space, persist it and switch to it.

use super::super::view::AppView;
use crate::state::{NetworkNameError, MAX_NETWORK_NAME_CHARS};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Notification id of the "created" toast.
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

/// The last rejected name and why, shown under the field until the name is edited (T20: an
/// error belongs next to what caused it, not in a toast across the window).
type NameProblem = Rc<RefCell<Option<(String, String)>>>;

impl AppView {
    /// Closes the workspace popover and asks for the new network's name.
    pub(crate) fn open_create_network(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.chrome.workspace_menu.close();
        let placeholder = t!("workspace.create_placeholder").to_string();
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let view = cx.entity().downgrade();
        let needs_focus = Cell::new(true);
        let problem: NameProblem = Rc::new(RefCell::new(None));
        let (size, colour) = (self.theme.font_size_xs, self.theme.accent_danger);
        window.open_alert_dialog(cx, move |alert, window, cx| {
            if needs_focus.replace(false) {
                input.update(cx, |state, cx| state.focus(window, cx));
            }
            let current = input.read(cx).value().to_string();
            let message = problem.borrow().as_ref().filter(|(about, _)| *about == current).map(|(_, message)| message.clone());
            let (input, view, problem) = (input.clone(), view.clone(), problem.clone());
            alert
                .confirm()
                .title(t!("workspace.create_title").to_string())
                .description(t!("workspace.create_desc").to_string())
                .child(Input::new(&input))
                .children(message.map(|message| div().text_size(size).text_color(colour).child(message)))
                .ok_text(t!("workspace.create_button").to_string())
                .cancel_text(t!("composer.cancel").to_string())
                .on_ok(move |_, window, cx| {
                    let name = input.read(cx).value().to_string();
                    match view.update(cx, |this, cx| this.create_network_named(&name, window, cx)) {
                        Ok(Err(message)) => {
                            *problem.borrow_mut() = Some((name, message));
                            window.refresh();
                            false
                        }
                        _ => true,
                    }
                })
        });
        cx.notify();
    }

    /// Creates and switches to the network, or says why the name was refused (the dialog then
    /// stays open with the reason under the field).
    fn create_network_named(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) -> Result<(), String> {
        let id = self.state.create_network(name).map_err(|err| network_name_message(&err))?;
        let note = t!("workspace.created", name = name.trim()).to_string();
        window.push_notification(Notification::success(note).id::<CreateNetworkNotice>(), cx);
        self.select_network(id, window, cx);
        Ok(())
    }
}

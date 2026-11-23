//! The confirm before code that the user did not write runs (P5): who wrote it, the runtime
//! it runs in, a warning that it runs on this computer with the user's own rights, and the
//! whole code, so nothing runs unseen. It reuses the app's alert dialog.

use super::super::view::AppView;
use super::code_run::RunAsk;
use crate::theme::ThemeTokens;
use crate::ui::features::post::render_code_block;
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::cell::RefCell;
use std::rc::Rc;

/// Opens the confirm; Run starts the run through `view`, anything else drops it.
pub(super) fn open_run_confirm(view: WeakEntity<AppView>, ask: RunAsk, theme: ThemeTokens, window: &mut Window, cx: &mut App) {
    let RunAsk { request, found, author } = ask;
    let intro = t!("code.confirm.from", author = author.as_str(), runtime = found.runtime.label()).to_string();
    let pending = Rc::new(RefCell::new(Some((request, found))));
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (code, lang) = pending.borrow().as_ref().map(|(request, found)| (request.code.clone(), found.runtime.key())).unwrap_or_default();
        let warning = div().text_color(theme.accent_danger).child(t!("code.confirm.warning").to_string());
        let body = div().flex().flex_col().gap(theme.stack_gap).child(intro.clone()).child(warning).child(render_code_block(0, Some(lang), &code, &theme));
        let (accepted, view) = (pending.clone(), view.clone());
        alert
            .confirm()
            .title(t!("code.confirm.title").to_string())
            .description(body)
            .ok_text(t!("code.confirm.run").to_string())
            .ok_variant(ButtonVariant::Danger)
            .cancel_text(t!("composer.cancel").to_string())
            .on_ok(move |_, window, cx| {
                if let Some((request, found)) = accepted.borrow_mut().take() {
                    let _ = view.update(cx, |this, cx| this.start_code_run(request, found, window, cx));
                }
                true
            })
    });
}

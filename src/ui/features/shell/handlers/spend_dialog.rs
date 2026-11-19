//! The "Ask before spending" confirm (sb P16): how many AI requests the action sends, what is
//! left of today's budget, and "Don't ask again this session". It reuses the app's alert
//! dialog, like the delete confirmations.

use super::super::view::AppView;
use super::spend::SpendResume;
use crate::domain::spend::SpendEstimate;
use crate::i18n::t_count;
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const REMEMBER_ID: &str = "spend-confirm-remember";

/// One confirm: what it asks for, and what runs when it is accepted or closed without sending.
pub(super) struct SpendAsk {
    pub estimate: SpendEstimate,
    /// Requests today's budget still allows (`None` without a daily limit).
    pub left: Option<u32>,
    /// Gap between the sentence and the checkbox.
    pub gap: Pixels,
    pub resume: SpendResume,
    pub decline: Option<SpendResume>,
}

/// The sentence under the title: the request count, then what today's budget has left.
pub(super) fn spend_summary(estimate: SpendEstimate, left: Option<u32>) -> String {
    let requests = t_count("spend.confirm.requests", estimate.requests as usize);
    match left {
        Some(left) => format!("{requests} {}", t_count("spend.confirm.left", left as usize)),
        None => requests,
    }
}

/// Opens the confirm; its answer goes back to the shell through `view`.
pub(super) fn open_spend_dialog(view: WeakEntity<AppView>, ask: SpendAsk, window: &mut Window, cx: &mut App) {
    let SpendAsk { estimate, left, gap, resume, decline } = ask;
    let (remember, resume, decline) = (Rc::new(Cell::new(false)), Rc::new(RefCell::new(Some(resume))), Rc::new(RefCell::new(decline)));
    let summary = spend_summary(estimate, left);
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (ok_view, close_view, accepted, pending, declined) = (view.clone(), view.clone(), resume.clone(), resume.clone(), decline.clone());
        let (tick, checked) = (remember.clone(), remember.clone());
        let remember_box = Checkbox::new(REMEMBER_ID).checked(checked.get()).label(t!("spend.confirm.remember").to_string()).on_click(move |on, window, _| {
            tick.set(*on);
            window.refresh();
        });
        let body = div().flex().flex_col().gap(gap).child(summary.clone()).child(remember_box);
        alert
            .confirm()
            .title(t!("spend.confirm.title").to_string())
            .description(body)
            .ok_text(t!("spend.confirm.send").to_string())
            .ok_variant(ButtonVariant::Primary)
            .cancel_text(t!("composer.cancel").to_string())
            .on_ok(move |_, window, cx| {
                if let Some(resume) = accepted.borrow_mut().take() {
                    let remembered = checked.get();
                    let _ = ok_view.update(cx, |this, cx| this.accept_spend(estimate, remembered, resume, window, cx));
                }
                true
            })
            .on_close(move |_, window, cx| {
                // Closed without sending (Cancel, Escape): the action that asked is told so.
                let declined = pending.borrow_mut().take().and_then(|_| declined.borrow_mut().take());
                let _ = close_view.update(cx, |this, cx| this.close_spend(declined, window, cx));
            })
    });
}

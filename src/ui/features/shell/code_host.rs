//! Settings › Code in the shell (P1): the cached runtime detection, and Refresh, which probes
//! every runtime on the background executor and stores what it found.

use super::binder::Binder;
use super::view::AppView;
use crate::domain::code::RuntimeCache;
use crate::state::AppState;
use crate::ui::features::settings::{RuntimeAction, RuntimesProps};
use gpui_kit::*;
use std::rc::Rc;

/// What Settings › Code shows.
#[derive(Debug, Clone, Default)]
pub struct CodeHost {
    pub cache: RuntimeCache,
    /// A detection is running.
    pub detecting: bool,
}

impl CodeHost {
    pub(super) fn new(state: &AppState) -> Self {
        Self { cache: state.runtime_cache(), detecting: false }
    }
}

impl AppView {
    pub(super) fn runtimes_props(&self, bind: &Binder) -> RuntimesProps {
        RuntimesProps { cache: self.code.cache.clone(), detecting: self.code.detecting, on_action: Rc::new(bind.with(Self::on_runtime_action)) }
    }

    fn on_runtime_action(&mut self, action: RuntimeAction, window: &mut Window, cx: &mut Context<Self>) {
        match action {
            RuntimeAction::Refresh => self.refresh_runtimes(window, cx),
        }
    }

    /// Probes every runtime off the UI thread; only one detection runs at a time.
    fn refresh_runtimes(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.code.detecting {
            return;
        }
        self.code.detecting = true;
        cx.notify();
        let task = cx.background_executor().spawn(async { crate::runner::detect_all() });
        cx.spawn_in(window, async move |this, cx| {
            let cache = task.await;
            let _ = this.update(cx, |this, cx| {
                this.state.store_runtime_cache(&cache);
                this.code = CodeHost { cache, detecting: false };
                cx.notify();
            });
        })
        .detach();
    }
}

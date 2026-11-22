//! Settings › Code in the shell (P1, P4): the cached runtime detection, Refresh, which probes
//! every runtime on the background executor and stores what it found, and the runtime switches.

use super::binder::Binder;
use super::view::AppView;
use crate::domain::code::RuntimeCache;
use crate::state::AppState;
use crate::ui::features::settings::{RuntimeAction, RuntimesProps};
use gpui_kit::*;
use crate::domain::code::Runtime;
use crate::ui::features::post::CodeActions;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// What Settings › Code shows.
#[derive(Debug, Clone, Default)]
pub struct CodeHost {
    pub cache: RuntimeCache,
    /// A detection is running.
    pub detecting: bool,
    /// P5: the runs under way, by post and block, with their cancel flags.
    pub running: HashMap<(String, usize), Arc<AtomicBool>>,
}

impl CodeHost {
    pub(super) fn new(state: &AppState) -> Self {
        Self { cache: state.runtime_cache(), ..Self::default() }
    }
}

impl AppView {
    pub(super) fn runtimes_props(&self, bind: &Binder) -> RuntimesProps {
        let prefs = self.state.preferences().code.clone();
        RuntimesProps { cache: self.code.cache.clone(), prefs, detecting: self.code.detecting, on_action: Rc::new(bind.with(Self::on_runtime_action)) }
    }

    fn on_runtime_action(&mut self, action: RuntimeAction, window: &mut Window, cx: &mut Context<Self>) {
        match action {
            RuntimeAction::Refresh => self.refresh_runtimes(window, cx),
            RuntimeAction::Switch(runtime, on) => {
                let prefs = self.state.preferences().clone();
                let code = prefs.code.clone().with_runtime(runtime, on);
                self.apply_preferences(crate::domain::preferences::Preferences { code, ..prefs }, window, cx);
            }
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
                (this.code.cache, this.code.detecting) = (cache, false);
                cx.notify();
            });
        })
        .detach();
    }
}

impl AppView {
    /// Run on code blocks (P5): the runtimes that are switched on and were found; `None` when
    /// none is (code running off, nothing detected), so no block shows Run.
    pub(super) fn code_actions(&self, bind: &Binder) -> Option<CodeActions> {
        let prefs = &self.state.preferences().code;
        let runnable: Vec<Runtime> = self.code.cache.found.iter().map(|found| found.runtime).filter(|runtime| prefs.allows(*runtime)).collect();
        (!runnable.is_empty()).then(|| CodeActions { runnable: runnable.into(), on_run: Rc::new(bind.with(Self::on_run_code)) })
    }
}

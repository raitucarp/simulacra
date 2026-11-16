//! [`ForgeHandle`]: turns `ForgeView` methods into the `'static` callbacks widgets accept.

use super::ForgeView;
use crate::state::forge::DraftEdit;
use crate::ui::features::forge::widgets::{ActionCallback, ValueCallback};
use gpui_kit::*;
use std::rc::Rc;

/// A weak handle on the forge that builds UI callbacks.
#[derive(Clone)]
pub struct ForgeHandle(WeakEntity<ForgeView>);

impl ForgeHandle {
    pub fn new(cx: &Context<ForgeView>) -> Self {
        Self(cx.entity().downgrade())
    }

    /// A no-argument callback running `handler` on the forge.
    pub fn act(&self, handler: impl Fn(&mut ForgeView, &mut Window, &mut Context<ForgeView>) + 'static) -> ActionCallback {
        let forge = self.0.clone();
        Rc::new(move |window, cx| {
            let _ = forge.update(cx, |this, cx| handler(this, window, cx));
        })
    }

    /// A one-argument callback forwarding its argument to `handler`.
    pub fn with<A: 'static>(
        &self,
        handler: impl Fn(&mut ForgeView, A, &mut Window, &mut Context<ForgeView>) + 'static,
    ) -> ValueCallback<A> {
        let forge = self.0.clone();
        Rc::new(move |arg, window, cx| {
            let _ = forge.update(cx, |this, cx| handler(this, arg, window, cx));
        })
    }

    /// A callback applying the edit `make` builds from its argument.
    pub fn edit<A: 'static>(&self, make: impl Fn(A) -> DraftEdit + 'static) -> ValueCallback<A> {
        self.with(move |this, arg, _, cx| this.apply_edit(make(arg), cx))
    }

    /// A callback ending the current text-typing undo run.
    pub fn seal(&self) -> ActionCallback {
        self.act(|this, _, _| this.draft.seal_history())
    }
}

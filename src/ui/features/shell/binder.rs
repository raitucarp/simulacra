//! Turns `AppView` methods into the boxed `(…, &mut Window, &mut App)` callbacks that
//! feature components accept, replacing one cloned entity handle per callback.

use super::view::AppView;
use gpui_kit::*;

/// A boxed no-argument UI callback.
pub(super) type BoxedAction = Box<dyn Fn(&mut Window, &mut App) + 'static>;
/// A boxed one-argument UI callback.
pub(super) type BoxedHandler<A> = Box<dyn Fn(A, &mut Window, &mut App) + 'static>;

/// Builds UI callbacks that update the root view through its entity handle.
#[derive(Clone)]
pub(super) struct Binder(Entity<AppView>);

impl Binder {
    pub(super) fn new(cx: &Context<AppView>) -> Self {
        Self(cx.entity().clone())
    }

    /// A binder for callbacks built outside the root view's own render (cached panels).
    pub(super) fn from_entity(entity: Entity<AppView>) -> Self {
        Self(entity)
    }

    /// A no-argument callback running `handler` on the view.
    pub(super) fn act(
        &self,
        handler: impl Fn(&mut AppView, &mut Window, &mut Context<AppView>) + 'static,
    ) -> impl Fn(&mut Window, &mut App) + 'static {
        let entity = self.0.clone();
        move |window, cx| entity.update(cx, |this, cx| handler(this, window, cx))
    }

    /// A one-argument callback forwarding its argument to `handler`.
    pub(super) fn with<A: 'static>(
        &self,
        handler: impl Fn(&mut AppView, A, &mut Window, &mut Context<AppView>) + 'static,
    ) -> impl Fn(A, &mut Window, &mut App) + 'static {
        let entity = self.0.clone();
        move |arg, window, cx| entity.update(cx, |this, cx| handler(this, arg, window, cx))
    }

    /// [`Self::act`], boxed.
    pub(super) fn boxed_act(
        &self,
        handler: impl Fn(&mut AppView, &mut Window, &mut Context<AppView>) + 'static,
    ) -> BoxedAction {
        Box::new(self.act(handler))
    }

    /// [`Self::with`], boxed.
    pub(super) fn boxed<A: 'static>(
        &self,
        handler: impl Fn(&mut AppView, A, &mut Window, &mut Context<AppView>) + 'static,
    ) -> BoxedHandler<A> {
        Box::new(self.with(handler))
    }

    /// The underlying view handle, for callbacks with borrowed arguments.
    pub(super) fn entity(&self) -> &Entity<AppView> {
        &self.0
    }
}

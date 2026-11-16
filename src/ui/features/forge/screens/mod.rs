//! Rendering of the forge screens: resume prompt, class picker, the step-by-step creator and
//! the celebration. Each screen is an `impl ForgeView` block in its own module.

mod celebrate;
mod common;
mod guide;
mod resume;
mod start;
mod steps;

use super::view::{ForgeHandle, ForgeScreen, ForgeView, FORGE_KEY_CONTEXT};
use gpui_kit::*;

const ROOT_ID: &str = "agent-forge-root";

impl Render for ForgeView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let handle = ForgeHandle::new(cx);
        let body = match &self.screen {
            ForgeScreen::Resume(saved) => self.resume_screen(saved, &handle),
            ForgeScreen::Start => self.start_screen(&handle),
            ForgeScreen::Steps => self.steps_screen(&handle),
            ForgeScreen::Celebrate => self.celebrate_screen(&handle),
        };
        let theme = &self.theme;
        div()
            .id(ROOT_ID)
            .key_context(FORGE_KEY_CONTEXT)
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::on_undo))
            .on_action(cx.listener(Self::on_redo))
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(theme.bg_app)
            .font_family(theme.font_family_ui.clone())
            .text_color(theme.text_primary)
            .child(body)
    }
}

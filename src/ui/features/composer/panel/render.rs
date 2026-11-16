//! Renders the panel through the stateless [`ComposerModalView`] dialog.

use super::super::modal::ComposerModalView;
use super::ComposerModalPanel;
use gpui_kit::*;

type PanelAction = fn(&mut ComposerModalPanel, &mut Window, &mut Context<ComposerModalPanel>);

/// Wraps a panel method as a `(Window, App)` callback.
fn act(entity: &Entity<ComposerModalPanel>, action: PanelAction) -> impl Fn(&mut Window, &mut App) + 'static {
    let entity = entity.clone();
    move |window, cx| entity.update(cx, |this, cx| action(this, window, cx))
}

impl Render for ComposerModalPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let draft_length = self.composer_input.read(cx).value().chars().count();
        ComposerModalView::new(
            self.theme.clone(),
            self.profile.clone(),
            self.mode.clone(),
            self.composer_input.clone(),
            self.link_input.clone(),
            self.image_input.clone(),
            self.image_path.clone(),
            self.is_link_input_open,
            self.is_image_input_open,
            draft_length,
            act(&entity, Self::close),
            act(&entity, Self::submit),
            act(&entity, Self::toggle_link),
            act(&entity, Self::toggle_image),
            act(&entity, Self::pick_image),
            act(&entity, Self::clear_image),
            act(&entity, Self::clear_link),
        )
    }
}

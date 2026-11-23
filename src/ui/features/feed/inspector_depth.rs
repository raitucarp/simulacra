//! N5: the depth dial under the focused post in the inspector, with the rendering of the
//! version it shows at the chosen depth (read only for that depth).

use super::inspector::InspectorContext;
use crate::domain::Post;
use crate::ui::features::data::render_depth_dial;
use gpui_kit::Div;

/// The dial of the focused `post`; `None` without data controls or for a post never rendered.
pub(super) fn render_depth(ctx: &InspectorContext<'_>, post: &Post) -> Option<Div> {
    let source = ctx.callbacks.data.as_ref()?;
    let chosen = source.depth.chosen.get(&post.id).copied();
    let rendering = chosen.and_then(|depth| ctx.state.rendering_of(&post.id, depth));
    render_depth_dial(post, rendering.as_ref(), &source.depth, &source.on_event, ctx.theme)
}

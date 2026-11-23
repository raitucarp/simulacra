//! N5: the depth dial under the focused post: "Simpler", "As written" and "Expert". Picking
//! a depth shows the agent's rendering of the version the post shows (kept as a version of
//! the post), or offers to ask for one; a rendering never replaces the post. A locked post
//! has no dial: its sealed text is never sent to an agent.

use super::events::{DataAction, DataEvent};
use super::sheet::notice;
use crate::domain::data::history::Revision;
use crate::domain::post::depth::prompt::can_render;
use crate::domain::post::depth::Depth;
use crate::domain::{Post, PostId};
use crate::theme::ThemeTokens;
use crate::ui::features::composer::version_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

const DIAL_ID: &str = "post-depth-";
const ASK_ID: &str = "post-depth-ask-";

/// What the dial shows for every post.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DepthView {
    /// The depth each post is read at (missing: as written).
    pub chosen: HashMap<PostId, Depth>,
    /// The rendering being asked for.
    pub running: Option<(PostId, Depth)>,
    /// Why the last request for a post failed.
    pub problems: HashMap<PostId, String>,
}

/// The translated name of `depth` (`None`: as written).
pub fn depth_label(depth: Option<Depth>) -> String {
    let key = format!("posts.depth.{}", depth.map_or("as_written", Depth::key));
    t!(&key).to_string()
}

fn dial_button(post: &PostId, depth: Option<Depth>, chosen: Option<Depth>, on_event: &DataAction) -> Button {
    let (on_event, post_id) = (on_event.clone(), post.clone());
    let part = depth.map_or("as_written", Depth::key);
    Button::new(SharedString::from(format!("{DIAL_ID}{part}-{post}"))).ghost().xsmall().selected(depth == chosen).label(depth_label(depth)).on_click(move |_, window, cx| on_event(DataEvent::ShowDepth(post_id.clone(), depth), window, cx))
}

fn muted(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text)
}

/// The rendering at `depth`, or the button asking for one; why the last request failed.
fn panel(post: &PostId, depth: Depth, rendering: Option<&Revision>, view: &DepthView, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let running = view.running.as_ref() == Some(&(post.clone(), depth));
    let (on_event, post_id) = (on_event.clone(), post.clone());
    let ask_key = format!("posts.depth.ask.{}", depth.key());
    let label = if running { t!("posts.depth.running").to_string() } else if rendering.is_some() { t!("posts.depth.again").to_string() } else { t!(&ask_key).to_string() };
    let ask = Button::new(SharedString::from(format!("{ASK_ID}{post}"))).ghost().xsmall().icon(Icon::new(LucideIcon::WandSparkles)).label(label).disabled(running).on_click(move |_, window, cx| on_event(DataEvent::RenderDepth(post_id.clone(), depth), window, cx));
    let shown = rendering.map(|r| {
        let title = t!("posts.depth.rendering", depth = depth_label(Some(depth)), version = version_label(r.version.unwrap_or_default())).to_string();
        let text = div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(r.content.clone());
        div().flex().flex_col().gap(theme.spacing_3xs).child(muted(title, theme)).child(text).child(muted(t!("posts.depth.note").to_string(), theme))
    });
    let problem = view.problems.get(post).map(|text| notice(LucideIcon::CircleAlert, text.clone(), theme.accent_danger, theme));
    div().flex().flex_col().gap(theme.spacing_xs).p(theme.spacing_sm).rounded(theme.radius_md).bg(theme.bg_callout).children(shown).children(problem).child(ask)
}

/// The dial of `post` with its `rendering` at the chosen depth; `None` for a post that is
/// never rendered (locked or empty).
pub fn render_depth_dial(post: &Post, rendering: Option<&Revision>, view: &DepthView, on_event: &DataAction, theme: &ThemeTokens) -> Option<Div> {
    if !can_render(post) {
        return None;
    }
    let chosen = view.chosen.get(&post.id).copied();
    let buttons = [Some(Depth::Simpler), None, Some(Depth::Expert)].map(|depth| dial_button(&post.id, depth, chosen, on_event));
    let dial = div().flex().items_center().gap(theme.spacing_3xs).child(Icon::new(LucideIcon::Gauge).xsmall().text_color(theme.text_muted)).children(buttons);
    let panel = chosen.map(|depth| panel(&post.id, depth, rendering, view, on_event, theme));
    Some(div().w_full().px(theme.panel_padding).pb(theme.spacing_sm).flex().flex_col().gap(theme.spacing_xs).child(dial).children(panel))
}

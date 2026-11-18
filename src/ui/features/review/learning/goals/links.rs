//! A goal's linked material: chips for every link (removable), a concept field, the vault's
//! documents, "Link a post" (the picker) and review items to link.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::items::ItemAction;
use super::super::render::{chip, framed_input, section_label};
use crate::domain::learning::goals::edit::{with_link, without_link};
use crate::domain::learning::{Goal, GoalLink};
use crate::domain::review::ItemId;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const LINK_ID_PREFIX: &str = "learning-goal-link-";
const DOCUMENT_ID_PREFIX: &str = "learning-goal-document-";
const ITEM_ID_PREFIX: &str = "learning-goal-item-";
const PICK_POST_ID: &str = "learning-goal-pick-post";
/// Characters of a linked post or item shown on its chip.
const CHIP_CHARS: usize = 40;
const ELLIPSIS: &str = "…";
/// Documents offered to link.
const DOCUMENT_CHIPS: usize = 6;

fn shortened(text: &str) -> String {
    let cut: String = text.chars().take(CHIP_CHARS).collect();
    if cut.chars().count() < text.chars().count() { format!("{cut}{ELLIPSIS}") } else { cut }
}

impl ReviewPanel {
    /// What a link's chip says.
    fn link_label(&self, link: &GoalLink) -> String {
        let text = match link {
            GoalLink::Post { id } | GoalLink::Thread { root_id: id } => self.data.post_text(id).unwrap_or_else(|| tr("review.source.missing")),
            GoalLink::Document { id } => self.data.document_title(id).cloned().unwrap_or_else(|| tr("review.source.missing")),
            GoalLink::Item { id } => self.data.item(id).map_or_else(|| tr("review.source.missing"), |item| self.item_line(item)),
            GoalLink::Concept { name } => name.clone(),
        };
        format!("{} · {}", tr(&format!("learning.link.{}", link.key())), shortened(&text))
    }

    fn link_chips(&self, goal: &Goal, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chips = goal.links.iter().enumerate().map(|(index, link)| {
            let removed = link.clone();
            chip(ElementId::NamedInteger(LINK_ID_PREFIX.into(), index as u64), self.link_label(link), true, theme)
                .flex()
                .items_center()
                .gap(theme.spacing_3xs)
                .child(Icon::new(LucideIcon::X).size(theme.icon_size_xs).text_color(theme.text_muted))
                .on_click(cx.listener(move |this, _, _, cx| this.change_goal(|goal| without_link(goal, &removed), cx)))
        });
        control_row(theme).children(chips)
    }

    pub(super) fn goal_links(&self, goal: &Goal, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let documents = self.data.learning.documents.iter().filter(|(id, _)| !goal.has_link(&GoalLink::Document { id: id.clone() })).take(DOCUMENT_CHIPS).enumerate().map(|(index, (id, title))| {
            let link = GoalLink::Document { id: id.clone() };
            chip(ElementId::NamedInteger(DOCUMENT_ID_PREFIX.into(), index as u64), shortened(title), false, theme).on_click(cx.listener(move |this, _, _, cx| this.change_goal(|goal| with_link(goal, link.clone()), cx)))
        }).collect::<Vec<_>>();
        let pick = Button::new(PICK_POST_ID).ghost().small().icon(Icon::new(LucideIcon::MessageSquare)).label(tr("learning.goals.link_post")).on_click(cx.listener(|this, _, _, cx| this.pick_goal_post(cx)));
        let linked: Vec<ItemId> = goal.links.iter().filter_map(|link| match link { GoalLink::Item { id } => Some(id.clone()), _ => None }).collect();
        let entity = cx.entity().downgrade();
        let on_item: ItemAction = Rc::new(move |id, _, cx| {
            let _ = entity.update(cx, |panel, cx| panel.change_goal(|goal| with_link(goal, GoalLink::Item { id }), cx));
        });
        let empty = goal.links.is_empty().then(|| caption(tr("learning.goals.no_links"), theme));
        surface(theme)
            .child(block_title(LucideIcon::Link2, theme.accent_brand, tr("learning.goals.links_title"), theme))
            .children(empty)
            .child(self.link_chips(goal, cx))
            .child(section_label(tr("learning.goals.add_concept"), theme))
            .child(control_row(theme).child(framed_input(&self.learn_fields.goal_concept, theme)).child(pick))
            .child(section_label(tr("learning.goals.add_document"), theme))
            .child(control_row(theme).children(documents))
            .child(section_label(tr("learning.goals.add_item"), theme))
            .child(self.render_item_chooser(ITEM_ID_PREFIX, &linked, on_item, cx))
    }
}

//! The node editor: the statement's text, its role, the post it cites (for a new one), and
//! for an existing node its standing, its links (flip support/attack, remove) and the
//! buttons that draw a new support or attack edge from it.

use super::super::labels::{edge_colour, edge_label, role_colour, role_label, standing_colour, standing_label, tr};
use super::{ArgumentPanel, EditorMode};
use crate::domain::thinking::argument::{ArgEdge, EdgeKind, NodeRole};
use crate::ui::components::choice_chip;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;

const ROLE_ID: &str = "argument-role";
const SOURCE_ID: &str = "argument-source";
const EDGE_ID: &str = "argument-edge";
const ACTION_IDS: [&str; 5] = ["argument-save", "argument-support", "argument-attack", "argument-remove", "argument-close"];

impl ArgumentPanel {
    /// The editor card, while a node is edited or a statement written.
    pub(super) fn render_editor(&self, cx: &mut Context<Self>) -> Option<Div> {
        let theme = &self.theme;
        let (role, title) = match &self.editor {
            EditorMode::Closed => return None,
            EditorMode::New { role, .. } => (*role, tr("thinking.argument.new_title")),
            EditorMode::Node(id) => (self.thread.as_ref()?.map.node(id)?.role, tr("thinking.argument.edit_title")),
        };
        let heading = div().flex().items_center().gap(theme.inline_gap).child(Icon::new(LucideIcon::Edit).size(theme.icon_size_sm).text_color(theme.accent_brand_text)).child(div().flex_1().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title));
        let close = Button::new(ACTION_IDS[4]).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("thinking.close")).on_click(cx.listener(|panel, _, _, cx| panel.close_editor(cx)));
        let field = div().flex().items_center().gap(theme.control_gap).child(div().flex_1().min_w_0().child(Input::new(&self.input).small())).child(Button::new(ACTION_IDS[0]).primary().xsmall().label(tr("thinking.save")).on_click(cx.listener(|panel, _, _, cx| panel.save_text(cx))));
        let card = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .p(theme.card_padding)
            .rounded(theme.radius_lg)
            .bg(theme.bg_card)
            .border(theme.border_width_hairline)
            .border_color(theme.accent_brand_border)
            .child(div().flex().items_center().child(div().flex_1().child(heading)).child(close))
            .child(field)
            .child(self.role_chips(role, cx));
        Some(match self.editor.clone() {
            EditorMode::New { source, .. } => card.child(self.source_chips(&source, cx)),
            EditorMode::Node(id) => card.children(self.node_details(&id, cx).unwrap_or_default()),
            EditorMode::Closed => card,
        })
    }

    fn role_chips(&self, current: NodeRole, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chips = NodeRole::ALL.iter().enumerate().map(|(i, role)| {
            let role = *role;
            let entity = cx.entity().downgrade();
            choice_chip(ElementId::NamedInteger(ROLE_ID.into(), i as u64), &role_label(role), role == current, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.set_role(role, cx));
            })
            .text_size(theme.font_size_2xs)
            .border_color(role_colour(role, theme))
        });
        div().flex().flex_wrap().gap(theme.spacing_xs).children(chips)
    }

    fn source_chips(&self, current: &crate::domain::PostId, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let posts = self.thread.as_ref().map(|t| t.posts.clone()).unwrap_or_default();
        let chips = posts.into_iter().enumerate().map(|(i, (id, author))| {
            let label = format!("P{} · {author}", i + 1);
            let (entity, selected, pick) = (cx.entity().downgrade(), &id == current, id.clone());
            choice_chip(ElementId::NamedInteger(SOURCE_ID.into(), i as u64), &label, selected, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.set_source(pick.clone(), cx));
            })
            .text_size(theme.font_size_2xs)
        });
        let label = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr("thinking.argument.cites"));
        div().flex().flex_wrap().items_center().gap(theme.spacing_xs).child(label).children(chips)
    }

    /// Standing, links and the link / remove buttons of an existing node.
    fn node_details(&self, id: &str, cx: &mut Context<Self>) -> Option<Vec<AnyElement>> {
        let theme = &self.theme;
        let thread = self.thread.as_ref()?;
        let index = thread.map.index_of(id)?;
        let standing = *self.report.standing.get(index)?;
        let badge = div().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_2xs).text_color(standing_colour(standing, theme)).child(Icon::new(LucideIcon::Scale).size(theme.icon_size_xs)).child(standing_label(standing));
        let edges: Vec<AnyElement> = thread.map.edges.iter().filter(|e| e.from == id || e.to == id).enumerate().map(|(i, e)| self.edge_row(i, e, cx).into_any_element()).collect();
        let link = |slot: usize, kind: EdgeKind, icon: LucideIcon| {
            Button::new(ACTION_IDS[slot]).ghost().xsmall().selected(self.linking == Some(kind)).icon(Icon::new(icon).text_color(edge_colour(kind, theme))).label(tr(&format!("thinking.argument.link_{}", kind.key()))).on_click(cx.listener(move |panel, _, _, cx| panel.toggle_linking(kind, cx)))
        };
        let remove = Button::new(ACTION_IDS[3]).ghost().xsmall().icon(Icon::new(LucideIcon::Trash)).label(tr("thinking.argument.remove")).on_click(cx.listener(|panel, _, _, cx| panel.remove_edited(cx)));
        let buttons = div().flex().flex_wrap().gap(theme.control_gap).child(link(1, EdgeKind::Support, LucideIcon::Plus)).child(link(2, EdgeKind::Attack, LucideIcon::Swords)).child(remove);
        Some([badge.into_any_element()].into_iter().chain(edges).chain([buttons.into_any_element()]).collect())
    }

    /// One edge of the edited node: its kind and other end, with flip and remove.
    fn edge_row(&self, slot: usize, edge: &ArgEdge, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let text = |id: &str| self.thread.as_ref().and_then(|t| t.map.node(id)).map(|n| n.text.clone()).unwrap_or_default();
        let label = format!("{} → {}", text(&edge.from), text(&edge.to));
        let (from, to, kind) = (edge.from.clone(), edge.to.clone(), edge.kind);
        let (from2, to2) = (from.clone(), to.clone());
        let flip = Button::new(ElementId::NamedInteger(EDGE_ID.into(), slot as u64 * 2)).ghost().xsmall().icon(Icon::new(LucideIcon::ArrowLeftRight)).tooltip(tr("thinking.argument.flip")).on_click(cx.listener(move |panel, _, _, cx| panel.flip_edge(from.clone(), to.clone(), kind, cx)));
        let cut = Button::new(ElementId::NamedInteger(EDGE_ID.into(), slot as u64 * 2 + 1)).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("thinking.argument.unlink")).on_click(cx.listener(move |panel, _, _, cx| panel.remove_edge(from2.clone(), to2.clone(), cx)));
        let tag = div().flex_none().px(theme.spacing_xs).rounded(theme.radius_sm).bg(edge_colour(kind, theme)).text_color(theme.on_accent).child(edge_label(kind));
        div().w_full().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_2xs).child(tag).child(div().flex_1().min_w_0().truncate().text_color(theme.text_secondary).child(label)).child(flip).child(cut)
    }
}

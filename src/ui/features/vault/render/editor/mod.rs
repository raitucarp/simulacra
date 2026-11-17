//! The add / edit form, by progressive disclosure: the basics first (title, kind, creators,
//! date, text), then publication, classification and abstract sections on demand.

mod controls;
mod sections;

use super::common::{card, section_heading};
use crate::domain::document::record::DocumentKind;
use crate::ui::features::vault::editor::fields::TextField;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::kind_label;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, IllustrationKind};
use controls::{frame, labelled};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Textarea;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const EDITOR_SCROLL_ID: &str = "vault-editor";
const CANCEL_ID: &str = "vault-editor-cancel";
const SAVE_ID: &str = "vault-editor-save";
const KIND_CHIPS_ID: &str = "vault-kind";

impl VaultPanel {
    pub(super) fn render_editor(&self, editing: bool, compact: bool, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let title = if editing { t!("vault.edit_document") } else { t!("vault.add_document") }.to_string();
        let column = div()
            .w_full()
            .max_w(theme.vault_editor_width)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_editor_bar(title, cx))
            .child(self.render_basics(compact, cx))
            .children(self.render_sections(compact, cx))
            .child(self.render_editor_footer(cx));
        div()
            .id(EDITOR_SCROLL_ID)
            .size_full()
            .overflow_y_scrollbar()
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .items_center()
            .child(column)
            .into_any_element()
    }

    fn render_editor_bar(&self, title: String, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(self.render_back_bar(cx))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(title))
    }

    fn render_basics(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let kind = self.choice_row(KIND_CHIPS_ID, DocumentKind::ALL, Some(self.editor.choices.kind), kind_label, |draft, kind| {
            if let Some(kind) = kind {
                draft.kind = kind;
            }
        }, cx);
        let who_when = pair(compact, theme)
            .child(self.text_field(TextField::Creators, Some(t!("vault.hint.creators_roles").to_string())))
            .child(self.text_field(TextField::Date, Some(t!("vault.hint.date_note").to_string())));
        let content = frame(Textarea::new(&self.editor.content).bordered(false), false, theme);
        card(theme)
            .child(section_heading(t!("vault.section.basics").to_string(), theme))
            .child(self.text_field(TextField::Title, None))
            .child(labelled(t!("vault.field.kind").to_string(), None, None, self.with_kind_art(kind), theme))
            .child(who_when)
            .child(labelled(t!("vault.field.content").to_string(), Some(t!("vault.hint.content_note").to_string()), None, content, theme))
    }

    /// The kind chips beside the chosen kind's illustration.
    fn with_kind_art(&self, chips: Div) -> Div {
        let art = Illustration::asset(self.editor.choices.kind.illustration(), IllustrationKind::TraitCard).width(self.theme.vault_kind_art_width);
        div().flex().items_center().gap(self.theme.stack_gap).child(art).child(chips.flex_1())
    }

    fn render_editor_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let cancel = Button::new(CANCEL_ID).ghost().small().label(t!("composer.cancel").to_string()).on_click(cx.listener(|this, _, _, cx| this.back(cx)));
        let save = Button::new(SAVE_ID)
            .primary()
            .icon(Icon::new(LucideIcon::Check))
            .label(t!("vault.save").to_string())
            .on_click(cx.listener(|this, _, _, cx| this.save_editor(cx)));
        div().flex().justify_end().gap(self.theme.control_gap).child(cancel).child(save.small())
    }
}

/// Two fields side by side (stacked on compact windows).
pub(super) fn pair(compact: bool, theme: &crate::theme::ThemeTokens) -> Div {
    let row = div().flex().gap(theme.stack_gap);
    if compact {
        row.flex_col()
    } else {
        row.items_start()
    }
}

//! Footer: the dry-run counts and the two ways to finish, "Just add to vault" and "Create
//! posts" (disabled until something is selected and a pseudonym has a name).

use super::super::session::ImportStage;
use super::ImportSheet;
use crate::domain::preferences::PostIdentity;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const VAULT_ONLY_ID: &str = "import-vault-only";
const CREATE_POSTS_ID: &str = "import-create-posts";

impl ImportSheet {
    /// `312 units → 340 posts in 12 threads`.
    fn summary(&self) -> String {
        let counts = self.session.counts;
        match self.session.stage {
            ImportStage::Ready if counts.units > 0 => {
                t!("import.summary", units = counts.units, posts = counts.posts, threads = counts.threads).to_string()
            }
            ImportStage::Ready => t!("import.summary_vault_only").to_string(),
            _ => String::new(),
        }
    }

    fn can_create_posts(&self) -> bool {
        let named = self.session.identity != PostIdentity::Pseudonym || self.has_pseudonym;
        self.session.stage == ImportStage::Ready && self.session.counts.posts > 0 && named
    }

    pub(super) fn footer(&self) -> Div {
        let theme = &self.theme;
        let ready = self.session.stage == ImportStage::Ready;
        let (vault_only, create) = (self.actions.on_apply.clone(), self.actions.on_apply.clone());
        div()
            .flex_none()
            .px(theme.panel_padding)
            .py(theme.spacing_md)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.control_gap)
            .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_muted).child(self.summary()))
            .child(
                div()
                    .flex()
                    .gap(theme.control_gap)
                    .child(
                        Button::new(VAULT_ONLY_ID)
                            .ghost()
                            .small()
                            .disabled(!ready)
                            .label(t!("import.vault_only").to_string())
                            .on_click(move |_, window, cx| vault_only(false, window, cx)),
                    )
                    .child(
                        Button::new(CREATE_POSTS_ID)
                            .primary()
                            .small()
                            .disabled(!self.can_create_posts())
                            .label(t!("import.create_posts").to_string())
                            .on_click(move |_, window, cx| create(true, window, cx)),
                    ),
            )
    }
}

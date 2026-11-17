//! Who the posts go out as: you, the anonymous account or a pseudonym typed here.

use super::super::labels::identity_label;
use super::ImportSheet;
use crate::domain::preferences::PostIdentity;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::Input;
use gpui_kit::*;
use rust_i18n::t;

const IDENTITY_ID: &str = "import-identity";
const IDENTITIES: [PostIdentity; 3] = [PostIdentity::Yourself, PostIdentity::Anonymous, PostIdentity::Pseudonym];

fn identity_icon(identity: PostIdentity) -> LucideIcon {
    match identity {
        PostIdentity::Yourself => LucideIcon::User,
        PostIdentity::Anonymous => LucideIcon::EyeOff,
        PostIdentity::Pseudonym => LucideIcon::VenetianMask,
    }
}

impl ImportSheet {
    pub(super) fn author(&self) -> Div {
        let theme = &self.theme;
        let pills = IDENTITIES.into_iter().enumerate().map(|(index, identity)| {
            let pick = self.actions.on_identity.clone();
            let segment = Segment {
                id: (IDENTITY_ID, index).into(),
                icon: identity_icon(identity),
                label: identity_label(identity),
                active: identity == self.session.identity,
                active_icon_color: theme.accent_brand,
            };
            segment_pill(segment, theme, move |window, cx| pick(identity, window, cx))
        });
        let pseudonym = (self.session.identity == PostIdentity::Pseudonym)
            .then(|| Input::new(&self.inputs.pseudonym).w_full().h(theme.settings_input_height).text_size(theme.font_size_label));
        let hint = match self.session.identity {
            PostIdentity::Yourself => t!("import.post_as_hint.yourself"),
            PostIdentity::Anonymous => t!("import.post_as_hint.anonymous"),
            PostIdentity::Pseudonym => t!("import.post_as_hint.pseudonym"),
        };
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.section_title("import.post_as_title"))
            .child(segmented_track(theme).children(pills))
            .children(pseudonym)
            .child(div().text_size(theme.font_size_hint).text_color(theme.text_muted).child(hint.to_string()))
    }
}

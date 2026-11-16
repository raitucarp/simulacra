//! Sub-tab "Profile & Vocation": avatar picker plus name / handle / role / vocation.

use crate::ui::atoms::render_profile_avatar;
use crate::ui::components::{card_stack, fill, labeled_input, section_card, split_row};
use crate::ui::features::agent_editor::steps::StepCtx;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const AVATAR_TITLE: &str = "AVATAR & PORTRAIT";
const AVATAR_SUBTITLE: &str =
    "Upload a local image file (PNG, JPG, WebP) or use the monogram avatar.";
const AVATAR_PATH_LABEL: &str = "Avatar File Path";
const UPLOAD_BUTTON_ID: &str = "upload-avatar-file-btn";
const CLEAR_BUTTON_ID: &str = "clear-avatar-file-btn";

const IDENTITY_TITLE: &str = "CORE IDENTITY & VOCATION";
const IDENTITY_SUBTITLE: &str =
    "Human name, @username handle, short role headline, and daily profession.";
const NAME_LABEL: &str = "Full Human Name";
const HANDLE_LABEL: &str = "Username (@handle)";
const ROLE_LABEL: &str = "Role / Short Headline";
const OCCUPATION_LABEL: &str = "Occupation / Vocation";
const SAVE_BUTTON_ID: &str = "save-identity-sub0-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    card_stack(ctx.theme).child(avatar_card(ctx)).child(identity_card(ctx))
}

fn avatar_card(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    let has_avatar = ctx.profile.avatar_path.is_some();
    section_card(AVATAR_TITLE, AVATAR_SUBTITLE, theme).child(
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_md)
            .child(render_profile_avatar(ctx.profile, theme.hero_avatar_size, theme))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .gap(theme.spacing_sm)
                    .child(fill(labeled_input(AVATAR_PATH_LABEL, ctx.inputs.avatar_path_input, theme)))
                    .child(upload_button(ctx))
                    .when(has_avatar, |s| s.child(clear_button(ctx))),
            ),
    )
}

fn upload_button(ctx: &StepCtx<'_>) -> Div {
    let on_pick = ctx.on_pick_avatar.clone();
    let prof_id = ctx.profile.id.clone();
    div().pt(ctx.theme.spacing_md).child(
        Button::new(UPLOAD_BUTTON_ID)
            .primary()
            .compact()
            .icon(Icon::new(LucideIcon::Image))
            .label(t!("lab.upload_avatar").to_string())
            .on_click(move |_, w, cx| on_pick(prof_id.clone(), w, cx)),
    )
}

fn clear_button(ctx: &StepCtx<'_>) -> Div {
    let on_clear = ctx.action(|p| p.avatar_path = None);
    div().pt(ctx.theme.spacing_md).child(
        Button::new(CLEAR_BUTTON_ID)
            .ghost()
            .compact()
            .icon(Icon::new(LucideIcon::X))
            .on_click(move |_, w, cx| on_clear(w, cx)),
    )
}

fn identity_card(ctx: &StepCtx<'_>) -> Div {
    let (theme, inputs) = (ctx.theme, ctx.inputs);
    section_card(IDENTITY_TITLE, IDENTITY_SUBTITLE, theme)
        .child(
            split_row(theme.spacing_md)
                .child(fill(labeled_input(NAME_LABEL, inputs.name_input, theme)))
                .child(fill(labeled_input(HANDLE_LABEL, inputs.handle_input, theme))),
        )
        .child(
            split_row(theme.spacing_md)
                .child(fill(labeled_input(ROLE_LABEL, inputs.description_input, theme)))
                .child(fill(labeled_input(OCCUPATION_LABEL, inputs.occupation_input, theme))),
        )
        .child(ctx.save_footer(SAVE_BUTTON_ID))
}

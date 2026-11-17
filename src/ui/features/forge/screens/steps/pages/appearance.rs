//! Appearance: switch between a generated portrait, a non-human form, a personality emblem and
//! a photo, then edit the chosen one (portrait parts in `portrait`, form parts in
//! `form_editor`, photo framing in `photo`).

use super::avatar_kind::AvatarKind;
use super::super::super::common::{card, heading, muted};
use crate::domain::portrait::{suggested_outfit, AvatarMode, EmblemSpec, FormKind, FormSpec, PortraitSpec};
use crate::state::forge::{AvatarField, DraftEdit};
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::features::forge::logic::time_seed;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ExplainerButton;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::PortraitView;
use gpui_kit::component::button::Button;
use gpui_kit::component::Disableable;
use gpui_kit::*;

const STYLE_KEY: &str = "forge.ui.appearance.style";
const NONE_KEY: &str = "forge.ui.appearance.no_style";
const EMBLEM_HINT_KEY: &str = "forge.ui.appearance.emblem_hint";
const REDERIVE_KEY: &str = "forge.ui.appearance.rederive";
const MODE_ID_PREFIX: &str = "forge-avatar-mode";
const REDERIVE_ID: &str = "forge-emblem-rederive";
/// The emblem preview is this fraction of the stage portrait.
const PREVIEW_SCALE: f32 = 0.5;

impl ForgeView {
    pub(in super::super) fn appearance_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let mode = self.draft.profile().config.avatar.clone();
        let body = match &mode {
            Some(AvatarMode::Pcg(spec)) => self.portrait_editor(*spec, handle).into_any_element(),
            Some(AvatarMode::Form(spec)) => self.form_editor(spec, handle).into_any_element(),
            Some(AvatarMode::Emblem(spec)) => self.emblem_panel(*spec, handle).into_any_element(),
            Some(AvatarMode::Photo { crop, .. }) => self.photo_panel(Some(*crop), handle).into_any_element(),
            None => muted(tr(NONE_KEY), theme).into_any_element(),
        };
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .child(card(theme).child(self.style_heading()).child(self.mode_switch(AvatarKind::of(mode.as_ref()), handle)))
            .child(body)
            .into_any_element()
    }

    /// "Avatar style" with the avatar field's explainer.
    fn style_heading(&self) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(heading(tr(STYLE_KEY), theme))
            .children(ExplainerButton::for_field(AvatarField::Avatar.path(), theme))
    }

    fn mode_switch(&self, current: Option<AvatarKind>, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let pills = AvatarKind::ALL.map(|kind| {
            let pick = handle.act(move |this, window, cx| this.switch_avatar(kind, window, cx));
            let segment = Segment {
                id: (MODE_ID_PREFIX, kind as usize).into(),
                icon: kind.icon(),
                label: tr(kind.label_key()),
                active: current == Some(kind),
                active_icon_color: theme.accent_brand,
            };
            segment_pill(segment, theme, move |w, cx| pick(w, cx))
        });
        segmented_track(theme).children(pills)
    }

    /// Switches the avatar kind, keeping the current one when it is already that kind.
    pub(super) fn switch_avatar(&mut self, kind: AvatarKind, window: &mut Window, cx: &mut Context<Self>) {
        let cfg = &self.draft.profile().config;
        if AvatarKind::of(cfg.avatar.as_ref()) == Some(kind) {
            return;
        }
        let mode = match kind {
            AvatarKind::Portrait => {
                let spec = PortraitSpec::random(time_seed());
                AvatarMode::Pcg(PortraitSpec { outfit: suggested_outfit(&cfg.occupation).unwrap_or(spec.outfit), ..spec })
            }
            AvatarKind::Form => AvatarMode::Form(FormSpec::random_any(FormKind::default(), time_seed())),
            AvatarKind::Emblem => AvatarMode::Emblem(EmblemSpec::for_config(cfg)),
            AvatarKind::Photo => return self.pick_photo(window, cx),
        };
        self.apply_edit(DraftEdit::Avatar(AvatarField::Avatar, Some(mode)), cx);
    }

    fn emblem_panel(&self, spec: EmblemSpec, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let derived = EmblemSpec::for_config(&self.draft.profile().config);
        let rederive = handle.act(move |this, _, cx| this.apply_edit(DraftEdit::Avatar(AvatarField::Avatar, Some(AvatarMode::Emblem(derived))), cx));
        card(theme)
            .items_center()
            .child(PortraitView::new(AvatarMode::Emblem(spec), theme.forge_portrait_size * PREVIEW_SCALE, theme))
            .child(muted(tr(EMBLEM_HINT_KEY), theme))
            .child(
                Button::new(REDERIVE_ID)
                    .outline()
                    .icon(LucideIcon::RefreshCw)
                    .label(tr(REDERIVE_KEY))
                    .disabled(derived == spec)
                    .on_click(move |_, w, cx| rederive(w, cx)),
            )
    }
}

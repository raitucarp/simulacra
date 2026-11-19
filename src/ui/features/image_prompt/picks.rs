//! The left column: one section of art chips per kind of pick (style, mood, palette,
//! framing for the target, and what to keep out).

use super::chip::art_chip;
use super::labels::tr;
use super::model::{IdeasEvent, IdeasPick, IdeasSheet};
use super::sheet::IdeasAction;
use crate::domain::image_prompt::{ImageAvoid, ImageMood, ImagePalette, ImageStyle};
use crate::domain::TraitOption;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

const SCROLL_ID: &str = "image-ideas-picks";
const CHIP_ID_PREFIX: &str = "image-ideas-chip";

/// One labelled row of chips.
struct Section {
    title_key: &'static str,
    picks: Vec<IdeasPick>,
}

fn sections(sheet: &IdeasSheet) -> [Section; 5] {
    [
        Section { title_key: "section.style", picks: ImageStyle::ALL.iter().map(|o| IdeasPick::Style(*o)).collect() },
        Section { title_key: "section.mood", picks: ImageMood::ALL.iter().map(|o| IdeasPick::Mood(*o)).collect() },
        Section { title_key: "section.palette", picks: ImagePalette::ALL.iter().map(|o| IdeasPick::Palette(*o)).collect() },
        Section { title_key: "section.framing", picks: sheet.context.target.framings().iter().map(|o| IdeasPick::Framing(*o)).collect() },
        Section { title_key: "section.avoid", picks: ImageAvoid::ALL.iter().map(|o| IdeasPick::Avoid(*o)).collect() },
    ]
}

/// (art path, label key) of a pick.
fn pick_art(pick: IdeasPick) -> (String, String) {
    match pick {
        IdeasPick::Style(o) => (o.illustration(), o.label_key()),
        IdeasPick::Mood(o) => (o.illustration(), o.label_key()),
        IdeasPick::Palette(o) => (o.illustration(), o.label_key()),
        IdeasPick::Framing(o) => (o.illustration(), o.label_key()),
        IdeasPick::Avoid(o) => (o.illustration(), o.label_key()),
    }
}

fn section_view(index: usize, section: Section, sheet: &IdeasSheet, on_event: &IdeasAction, theme: &ThemeTokens) -> Div {
    let chips = section.picks.into_iter().enumerate().map(|(at, pick)| {
        let (art, label_key) = pick_art(pick);
        let on_event = on_event.clone();
        let id = (SharedString::from(format!("{CHIP_ID_PREFIX}-{index}")), at);
        art_chip(id, art, t!(label_key.as_str()).to_string(), pick.is_on(&sheet.choices), theme, move |window, cx| {
            on_event(IdeasEvent::Pick(pick), window, cx)
        })
    });
    let title = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(tr(section.title_key));
    div().flex().flex_col().gap(theme.spacing_xs).child(title).child(div().flex().flex_wrap().gap(theme.spacing_xs).children(chips))
}

/// The scrollable column of pick sections.
pub(super) fn render_picks(sheet: &IdeasSheet, on_event: &IdeasAction, theme: &ThemeTokens) -> Stateful<Div> {
    let views = sections(sheet).into_iter().enumerate().map(|(index, section)| section_view(index, section, sheet, on_event, theme));
    div()
        .id(SCROLL_ID)
        .w(theme.image_ideas_picks_width)
        .flex_none()
        .h_full()
        .overflow_y_scroll()
        .p(theme.panel_padding)
        .flex()
        .flex_col()
        .gap(theme.section_gap)
        .border_r(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .children(views)
}

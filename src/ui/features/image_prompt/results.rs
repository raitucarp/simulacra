//! Generation: the "Generate" button (or why the engine cannot draw, with the way to fix it),
//! the drawing in progress, and the pictures so far, each with its "Set as avatar" or
//! "Attach to post" button.

use super::labels::{failure_art, failure_text, fixed_in_image_settings, tr, use_label};
use super::model::{GenerationStatus, IdeasEvent, IdeasSheet};
use super::prompt_pane::{label, notice};
use super::sheet::IdeasAction;
use crate::domain::image_prompt::ImageTarget;
use crate::theme::ThemeTokens;
use crate::ui::components::{art_state, ArtState};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration, NoticeKey};
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::path::PathBuf;

const GENERATE_ID: &str = "image-ideas-generate";
const STOP_ID: &str = "image-ideas-stop-drawing";
const SETTINGS_ID: &str = "image-ideas-settings";
const RESULT_ID: &str = "image-ideas-result";

/// The generation section of the pane.
pub(super) fn render_generation(sheet: &IdeasSheet, on_event: &IdeasAction, theme: &ThemeTokens) -> Div {
    let section = div().flex().flex_col().gap(theme.stack_gap).pt(theme.spacing_sm).border_t(theme.border_width_hairline).border_color(theme.border_subtle);
    if let Some(problem) = &sheet.engine_problem {
        let fix = fixed_in_image_settings(problem).then(|| send(Button::new(SETTINGS_ID).outline().small().icon(Icon::new(LucideIcon::Settings)).label(tr("open_settings")), IdeasEvent::OpenSettings, on_event));
        return section.child(notice(failure_art(problem), failure_text(problem), theme).children(fix));
    }
    let running = sheet.generation == GenerationStatus::Running;
    let generate = send(Button::new(GENERATE_ID).primary().small().icon(Icon::new(LucideIcon::ImagePlus)).label(tr("generate")).disabled(running), IdeasEvent::Generate, on_event);
    let status = match &sheet.generation {
        GenerationStatus::Idle => None,
        GenerationStatus::Running => {
            let stop = send(Button::new(STOP_ID).ghost().small().icon(Icon::new(LucideIcon::X)).label(tr("cancel")), IdeasEvent::Cancel, on_event);
            Some(notice(NoticeKey::Working, tr("generating"), theme).child(stop))
        }
        GenerationStatus::Failed(failure) => Some(notice(failure_art(failure), failure_text(failure), theme)),
    };
    section.child(div().flex().items_center().justify_between().child(label(tr("results_title"), theme)).child(generate)).children(status).child(results_grid(sheet, on_event, theme))
}

fn send(button: Button, event: IdeasEvent, on_event: &IdeasAction) -> Button {
    let on_event = on_event.clone();
    button.on_click(move |_, window, cx| on_event(event.clone(), window, cx))
}

/// The pictures so far, or the empty frames.
fn results_grid(sheet: &IdeasSheet, on_event: &IdeasAction, theme: &ThemeTokens) -> Div {
    if sheet.results.is_empty() {
        let key = if sheet.context.target == ImageTarget::Post { EmptyKey::ImageIdeas } else { EmptyKey::ImageResults };
        let art = Illustration::empty(key).width(theme.art_empty_width);
        return div().flex().justify_center().child(art_state(ArtState::hint(art, tr("results_empty")), theme));
    }
    let tiles = sheet.results.iter().enumerate().map(|(index, path)| {
        let use_it = send(Button::new((RESULT_ID, index)).outline().xsmall().icon(Icon::new(LucideIcon::Check)).label(use_label(&sheet.destination)), IdeasEvent::UseResult(index), on_event);
        let picture = div().size(theme.image_ideas_result_size).rounded(theme.radius_md).overflow_hidden().bg(theme.bg_card).child(img(PathBuf::from(path)).size_full().object_fit(ObjectFit::Cover));
        div().flex().flex_col().items_center().gap(theme.spacing_xs).child(picture).child(use_it)
    });
    div().flex().flex_wrap().gap(theme.stack_gap).children(tiles)
}

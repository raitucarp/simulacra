//! N12: the drawing sheet in the shell. The pad keeps the strokes; posting finishes the
//! drawing (clamped and simplified), saves it as SVG in the media folder (inks in the
//! theme's colours, the text ink as `currentColor`) and publishes it with the caption.

use super::super::view::AppView;
use crate::domain::post::ink::{Ink, SVG_TEXT_INK};
use crate::media::ink::save_ink_svg;
use crate::ui::components::ink_figure::{hex, ink_colour};
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, InkDialog, InkEvent, InkPad};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one drawing sheet event.
    pub(in crate::ui::features::shell) fn on_ink_event(&mut self, event: InkEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            InkEvent::Open => self.open_ink(window, cx),
            InkEvent::Pen(ink) => self.set_pen(cx, |pad, dialog| (pad.ink, dialog.ink) = (ink, ink)),
            InkEvent::Width(width) => self.set_pen(cx, |pad, dialog| (pad.width, dialog.width) = (width, width)),
            InkEvent::Undo => self.set_pen(cx, |pad, _| pad.undo()),
            InkEvent::Clear => self.set_pen(cx, |pad, _| pad.clear()),
            InkEvent::Submit => self.submit_ink(window, cx),
            InkEvent::Close => self.close_ink(window, cx),
        }
    }

    fn open_ink(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let caption = self.data.inputs.ideas.ink_caption.clone();
        caption.update(cx, |field, cx| field.set_value("", window, cx));
        let theme = self.theme.clone();
        let pad = cx.new(|_| InkPad::new(theme));
        let (ink, width) = { let pad = pad.read(cx); (pad.ink, pad.width) };
        self.data.ideas.ink = Some(InkDialog { pad, caption, ink, width, problem: None });
    }

    /// Changes the pad (and what the sheet shows of its pen).
    fn set_pen(&mut self, cx: &mut Context<Self>, change: impl FnOnce(&mut InkPad, &mut InkDialog)) {
        let Some(dialog) = self.data.ideas.ink.as_mut() else { return };
        let pad = dialog.pad.clone();
        pad.update(cx, |pad, cx| {
            change(pad, dialog);
            cx.notify();
        });
    }

    fn submit_ink(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.ink.as_ref() else { return };
        let Some(mut drawing) = dialog.pad.read(cx).drawing().finished() else {
            return self.ink_problem(tr("posts.ink.draw_first"));
        };
        let caption = dialog.caption.read(cx).value().to_string();
        let theme = self.theme.clone();
        let colour_of = move |ink: Ink| if ink == Ink::Text { SVG_TEXT_INK.to_string() } else { hex(ink_colour(ink, &theme)) };
        match save_ink_svg(&drawing, colour_of, &self.state.media_dir()) {
            Ok(path) => drawing.svg_path = Some(path),
            Err(reason) => return self.ink_problem(t!("posts.ink.not_saved", reason = reason).to_string()),
        }
        match self.state.submit_ink_post(&caption, drawing) {
            Ok(_) => {
                self.notify_data(tr("posts.ink.posted"), window, cx);
                self.close_ink(window, cx);
            }
            Err(err) => self.ink_problem(publish_error_message(&err)),
        }
    }

    fn ink_problem(&mut self, problem: String) {
        if let Some(dialog) = self.data.ideas.ink.as_mut() {
            dialog.problem = Some(problem);
        }
    }

    fn close_ink(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.inputs.ideas.ink_caption.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.ink = None;
    }
}

//! N3: planting, tending and returning garden notes from a post's menu. Planting tells the
//! user where the note went: it leaves the stream for the Garden view.

use super::super::view::AppView;
use crate::domain::post::garden::Maturity;
use crate::domain::PostId;
use crate::ui::features::data::tr;
use crate::ui::features::views::maturity_label;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Plants, tends or returns the post `id` (`None`: back to the stream).
    pub(in crate::ui::features::shell) fn set_garden(&mut self, id: &PostId, maturity: Option<Maturity>, window: &mut Window, cx: &mut Context<Self>) {
        let was_planted = self.state.get_post(id).is_some_and(|post| post.extras.garden.is_some());
        match self.state.set_garden(id, maturity, jiff::Timestamp::now()) {
            Ok(false) => {}
            Ok(true) => {
                let text = match (was_planted, maturity) {
                    (_, None) => tr("data.garden.returned"),
                    (false, Some(_)) => tr("data.garden.planted"),
                    (true, Some(stage)) => t!("data.garden.tended", stage = maturity_label(stage)).to_string(),
                };
                self.notify_data(text, window, cx);
                self.feed.stream.remeasure();
            }
            Err(err) => self.notify_data(crate::ui::features::composer::publish_error_message(&err), window, cx),
        }
    }
}

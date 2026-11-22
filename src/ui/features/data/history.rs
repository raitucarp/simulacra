//! The history sheet: every version of a post or a vault document drawn as a tree (V5,
//! newest edit first, graph lanes beside), and the word diff from the chosen version to the
//! current one, with "Restore this version" (and, for posts, "Edit from this version", V4).

use super::events::{DataAction, DataEvent};
use super::history_compare::comparison;
use super::history_rows::version_list_column;
use super::sheet::{footer, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::data::history::{current_index, lane_count, lane_rows, LaneRow, RevisionId, RevisionSubject, TreeNode, Version};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-history";
const RESTORE_ID: &str = "data-history-restore";
const CLOSE_ID: &str = "data-history-close";
const EDIT_FROM_ID: &str = "data-history-edit-from";

/// What the sheet shows.
#[derive(Debug, Clone, PartialEq)]
pub struct HistorySheet {
    pub subject: RevisionSubject,
    /// The post's opening words or the document's title.
    pub title: String,
    pub versions: Vec<Version>,
    pub selected: usize,
    /// The graph lanes of each version, and how many lanes the tree uses (V5).
    pub lanes: Vec<LaneRow>,
    pub lane_count: usize,
}

impl HistorySheet {
    /// The sheet on its most recent earlier version (the current one when there is none).
    pub fn new(subject: RevisionSubject, title: String, versions: Vec<Version>) -> Self {
        let current = current_index(&versions);
        let selected = (0..versions.len()).find(|&at| at != current).unwrap_or(current);
        let nodes: Vec<TreeNode<'_>> = versions.iter().map(|v| TreeNode { id: v.id.as_ref().map_or("", RevisionId::as_str), parent: v.parent.as_ref().map(RevisionId::as_str) }).collect();
        let lanes = lane_rows(&nodes);
        let lane_count = lane_count(&lanes);
        Self { subject, title, versions, selected, lanes, lane_count }
    }

    /// The version shown.
    pub fn current(&self) -> &Version {
        &self.versions[current_index(&self.versions)]
    }

    /// Whether the chosen version is the one shown (nothing to restore or edit from).
    pub fn chose_current(&self) -> bool {
        self.selected == current_index(&self.versions)
    }
}

fn buttons(sheet: &HistorySheet, on_event: DataAction, theme: &ThemeTokens) -> Div {
    let (restore, close, branch) = (on_event.clone(), on_event.clone(), on_event);
    let (selected, current) = (sheet.selected, sheet.chose_current());
    let is_post = matches!(sheet.subject, RevisionSubject::Post(_));
    footer(theme)
        .child(Button::new(CLOSE_ID).ghost().small().label(tr("data.close")).on_click(move |_, w, cx| close(DataEvent::CloseHistory, w, cx)))
        .when(is_post, |row| row.child(Button::new(EDIT_FROM_ID).ghost().small().icon(Icon::new(LucideIcon::GitBranch)).label(tr("data.history.edit_from")).disabled(current).on_click(move |_, w, cx| branch(DataEvent::EditFromVersion(selected), w, cx)))) // V4
        .child(Button::new(RESTORE_ID).primary().small().icon(Icon::new(LucideIcon::RotateCcw)).label(tr("data.history.restore")).disabled(current).on_click(move |_, w, cx| restore(DataEvent::RestoreVersion(selected), w, cx)))
}

/// The overlay.
pub fn render_history_sheet(sheet: &HistorySheet, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let list = version_list_column(sheet, &on_event, theme);
    let body = div().child(div().flex().flex_wrap().items_start().gap(theme.section_gap).child(list).child(div().flex_1().min_w(theme.history_list_width).child(comparison(sheet, theme))));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::RotateCcwClock,
        title: tr("data.history.title"),
        subtitle: Some(sheet.title.clone()),
        body,
        footer: buttons(sheet, on_event, theme),
        on_close: Rc::new(move |w, cx| closing(DataEvent::CloseHistory, w, cx)),
    };
    sheet_frame(parts, theme)
}

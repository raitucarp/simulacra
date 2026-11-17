//! Builds the palette's item snapshot from the store when it opens.

use super::model::{PaletteGroup, PaletteItem, PaletteTarget};
use super::registry::{CommandKind, CommandSpec, COMMANDS};
use crate::domain::{GroundingDocument, Post, Profile};
use crate::state::AppState;
use crate::ui::features::settings::SettingsTab;
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::shortcut_for;
use rust_i18n::t;

/// Characters of a post's first line used as its title, and of a body searched as keywords.
const POST_TITLE_CHARS: usize = 90;
const BODY_KEYWORD_CHARS: usize = 280;
/// Marks a shortened excerpt.
const ELLIPSIS: char = '…';
const SETTINGS_KEYWORDS_KEY: &str = "palette.settings_keywords";

/// Every searchable item of the active network, in group order.
pub fn collect_items(state: &AppState) -> Vec<PaletteItem> {
    let commands = COMMANDS.iter().filter(|spec| spec.is_listed()).map(command_item);
    let people = state.current_network_agents().into_iter().chain(state.current_network_personas()).map(profile_item);
    let posts = state.current_network_posts().map(|post| post_item(post, state.author_of(post)));
    let documents = state.current_documents().map(document_item);
    let settings = SettingsTab::ALL.into_iter().map(settings_item);
    commands.chain(people).chain(posts).chain(documents).chain(settings).collect()
}

fn item(target: PaletteTarget, group: PaletteGroup, title: String, icon: LucideIcon) -> PaletteItem {
    PaletteItem { target, group, title, detail: String::new(), keywords: String::new(), icon, shortcut: None }
}

fn command_item(spec: &CommandSpec) -> PaletteItem {
    let group = match spec.kind {
        CommandKind::Screen => PaletteGroup::Screens,
        CommandKind::Action => PaletteGroup::Commands,
    };
    PaletteItem {
        keywords: spec.keywords(),
        shortcut: shortcut_for(spec.id),
        ..item(PaletteTarget::Command(spec.id), group, spec.title(), spec.icon)
    }
}

fn profile_item(profile: &Profile) -> PaletteItem {
    let icon = if profile.is_agent() { LucideIcon::Bot } else { LucideIcon::User };
    PaletteItem {
        detail: profile.display_handle(),
        keywords: profile.handle.clone(),
        ..item(PaletteTarget::Profile(profile.id.clone()), PaletteGroup::People, profile.name.clone(), icon)
    }
}

fn post_item(post: &Post, author: &Profile) -> PaletteItem {
    let first_line = post.content.lines().find(|line| !line.trim().is_empty()).unwrap_or_default();
    PaletteItem {
        detail: author.name.clone(),
        keywords: excerpt(&post.content, BODY_KEYWORD_CHARS),
        ..item(PaletteTarget::Post(post.id.clone()), PaletteGroup::Posts, excerpt(first_line, POST_TITLE_CHARS), LucideIcon::MessageSquare)
    }
}

fn document_item(document: &GroundingDocument) -> PaletteItem {
    PaletteItem {
        keywords: excerpt(&document.content, BODY_KEYWORD_CHARS),
        ..item(PaletteTarget::Document(document.id.clone()), PaletteGroup::Documents, document.title.clone(), LucideIcon::FileText)
    }
}

fn settings_item(tab: SettingsTab) -> PaletteItem {
    PaletteItem {
        keywords: t!(SETTINGS_KEYWORDS_KEY).to_string(),
        ..item(PaletteTarget::Settings(tab), PaletteGroup::Settings, t!(tab.label_key()).to_string(), tab.icon())
    }
}

/// The first `max_chars` characters of `text`, marked when shortened.
pub fn excerpt(text: &str, max_chars: usize) -> String {
    let trimmed = text.trim();
    match trimmed.char_indices().nth(max_chars) {
        Some((cut, _)) => format!("{}{ELLIPSIS}", trimmed[..cut].trim_end()),
        None => trimmed.to_string(),
    }
}

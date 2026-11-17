//! Resolves pinned and recent [`NavRef`]s against the store into displayable rows; items that
//! no longer exist (or belong to another network) drop out.

use super::prefs::NavRef;
use crate::domain::Profile;
use crate::state::AppState;
use crate::ui::features::command_palette::excerpt;
use crate::ui::icons::LucideIcon;

/// Characters of a thread's first line shown as its sidebar label.
const THREAD_LABEL_CHARS: usize = 48;

/// A row of the Pinned or Recent section.
#[derive(Debug, Clone)]
pub struct ResolvedRef {
    pub reference: NavRef,
    pub label: String,
    pub icon: LucideIcon,
    /// The profile, for an avatar instead of the icon.
    pub profile: Option<Profile>,
}

/// The row of `reference` in the active network, if it still exists there.
pub fn resolve(state: &AppState, reference: &NavRef) -> Option<ResolvedRef> {
    let network = &state.active_network_id;
    let row = |label: String, icon, profile| ResolvedRef { reference: reference.clone(), label, icon, profile };
    match reference {
        NavRef::Thread(id) => state
            .get_post(id)
            .filter(|post| &post.network_id == network)
            .map(|post| row(thread_label(&post.content), LucideIcon::MessageSquare, None)),
        NavRef::Document(id) => state
            .documents()
            .iter()
            .find(|doc| &doc.id == id && &doc.network_id == network)
            .map(|doc| row(doc.title.clone(), LucideIcon::FileText, None)),
        NavRef::Profile(id) => state
            .user_profiles
            .iter()
            .find(|profile| &profile.id == id && profile.network_id.as_ref().is_none_or(|own| own == network))
            .map(|profile| row(profile.name.clone(), LucideIcon::User, Some(profile.clone()))),
    }
}

/// Every resolvable row of `references`, in order.
pub fn resolve_all(state: &AppState, references: &[NavRef]) -> Vec<ResolvedRef> {
    references.iter().filter_map(|reference| resolve(state, reference)).collect()
}

fn thread_label(content: &str) -> String {
    let first_line = content.lines().find(|line| !line.trim().is_empty()).unwrap_or_default();
    excerpt(first_line, THREAD_LABEL_CHARS)
}

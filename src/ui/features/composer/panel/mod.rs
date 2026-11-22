//! Stateful composer modal entity.
//!
//! Owns the thread being composed (one text area per post), attachment toggles and the mode,
//! re-rendering itself on keystrokes. The host only opens it and handles
//! [`ComposerModalEvent::Submit`], reporting store rejections back through [`ComposerModalPanel::set_error`].

mod actions;
mod atomic;
mod drafts;
mod edit; // sb F2: edit mode
mod files; mod format; mod host; mod media; mod preview; mod readability; pub mod readability_meter; mod render; mod schedule; mod symbols; mod version; mod free; mod todo; #[cfg(test)] mod tests; // T2: free compose; sb F3: the readability meter; posts B: formatting; D1: to-do help; L12: file chips

use super::consts::MODAL_LINK_PLACEHOLDER;
use super::modal::ComposerModalMode;
use crate::domain::media::ImageAttachment;
use crate::domain::post::Tint;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;

/// A thread the user asked to publish: one text per post, attachments on the first post.
#[derive(Debug, Clone, PartialEq)]
pub struct ComposerSubmission {
    pub mode: ComposerModalMode,
    pub drafts: Vec<String>,
    pub link: Option<String>,
    /// Attached images with their descriptions (sb C3), at most a gallery's worth.
    pub images: Vec<ImageAttachment>, pub files: Vec<crate::domain::media::FileRef>, // L12: the other files, linked where they are
    /// The container tint of every post of the thread, or of the edited post (sb C3).
    pub tint: Option<Tint>,
    /// When the thread is to go out (Y3); `None` posts it at once.
    pub publish_at: Option<jiff::Timestamp>,
    /// The symbols of the first post, or of the edited post (S5).
    pub symbols: crate::domain::symbols::PostSymbols,
    /// The version the edit of a published post makes (V3); `None` for new posts.
    pub edit: Option<crate::state::PostEdit>,
    /// The draft each post replies to when composing freely (T4: a tree); empty for a chain.
    pub replies_to: Vec<Option<usize>>,
}

/// Events emitted by [`ComposerModalPanel`].
#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalEvent {
    Submit(ComposerSubmission),
    /// sb D3: open image ideas for this draft text.
    ImageIdeas(String),
}

/// An attached image and the field holding its description (sb C3).
struct ComposerImage {
    attachment: ImageAttachment,
    alt: Entity<InputState>,
}

/// One post editor of the thread, with the subscription that keeps the panel in sync.
struct DraftSlot {
    input: Entity<TextareaState>,
    _subscription: Subscription,
}

/// Composer modal visibility, mode, thread drafts and attachments.
pub struct ComposerModalPanel {
    is_open: bool,
    mode: ComposerModalMode,
    drafts: Vec<DraftSlot>,
    /// Draft that last had focus; toolbar insertions target it.
    active_draft: usize,
    link_input: Entity<InputState>,
    image_input: Entity<InputState>,
    /// Images attached so far, and how many are still being copied into the media folder.
    images: Vec<ComposerImage>,
    pending_images: usize,
    media_dir: PathBuf,
    is_link_input_open: bool,
    is_image_input_open: bool,
    /// Last publish failure reported by the host, cleared on the next edit.
    error: Option<String>,
    theme: ThemeTokens,
    profile: Option<Profile>,
    atomic: atomic::AtomicHintState, // sb E1: the atomic-note hint
    readability: readability::ReadabilityState, // sb F3: the readability meter
    /// Drafts shown through the post body renderer instead of their editors.
    preview: bool, free: free::FreeCompose, // T2: the first editor is free compose text, one post per line
    tint: Option<Tint>, // sb C3: the picked container tint
    schedule: schedule::SchedulePicker, // Y3: the "Schedule…" picker
    symbols: symbols::SymbolPicker, // S5: the symbol picker
    version: version::VersionPicker, todo: todo::TodoAssist, files: files::ComposerFiles, // V3: the bump and note of an edit; D1: to-do help; L12: attached files
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ComposerModalEvent> for ComposerModalPanel {}

impl ComposerModalPanel {
    pub fn new(
        theme: ThemeTokens,
        profile: Option<Profile>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let link_input = cx.new(|cx| InputState::new(window, cx).placeholder(MODAL_LINK_PLACEHOLDER));
        let image_placeholder = t!("composer.image_path_placeholder").to_string();
        let image_input = cx.new(|cx| InputState::new(window, cx).placeholder(image_placeholder));
        let schedule = schedule::SchedulePicker::new(window, cx);
        let symbols = symbols::SymbolPicker::new(window, cx);
        let version = version::VersionPicker::new(window, cx);
        let mut panel = Self {
            is_open: false,
            mode: ComposerModalMode::NewThread,
            drafts: Vec::new(),
            active_draft: 0,
            link_input,
            image_input,
            images: Vec::new(),
            pending_images: 0,
            media_dir: crate::storage::media::media_dir_for(""),
            is_link_input_open: false,
            is_image_input_open: false,
            error: None,
            theme,
            profile,
            atomic: Default::default(), readability: Default::default(), preview: false, free: Default::default(), tint: None, schedule, symbols, version, todo: Default::default(), files: Default::default(), _subscriptions: Vec::new(),
        };
        panel._subscriptions = panel.subscribe(window, cx);
        panel.drafts.push(panel.new_draft_slot(window, cx));
        panel
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Opens the modal in `mode` with a single empty, focused draft.
    pub fn open(&mut self, mode: ComposerModalMode, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = mode;
        self.is_open = true;
        self.clear(window, cx);
        self.focus_draft(0, window, cx);
        cx.notify();
    }

    /// Closes the modal and clears its drafts.
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = false;
        self.clear(window, cx);
        cx.notify();
    }
}

//! Settings › AI Intelligence › Credentials: every stored provider / service key, with an
//! add / edit form. Secrets are shown masked; the global provider above stays the default.

mod editor;
mod list;
mod props;

pub use list::render_credentials_section;
pub use props::{CredentialEditorView, CredentialRowView, CredentialsPaneProps};

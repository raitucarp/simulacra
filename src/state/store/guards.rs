//! Guards of actions only the user may take: on behalf of one of their own personas (the
//! primary profile or an alter ego), never of an agent.

use crate::domain::Profile;
use crate::error::{AppError, Result};

/// `author` when it is one of the user's personas; an agent is refused with
/// [`AppError::NotAPersona`].
pub fn require_persona(author: &Profile) -> Result<&Profile> {
    if author.is_human_persona() {
        Ok(author)
    } else {
        Err(AppError::NotAPersona(author.id.to_string()))
    }
}

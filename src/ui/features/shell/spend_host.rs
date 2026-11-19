//! What the shell keeps for "Ask before spending" (sb P16): this session's consent, whether a
//! confirm is on screen, and the window the confirm opens in.

use crate::domain::spend::SpendConsent;
use gpui_kit::AnyWindowHandle;

/// The consent of the window's session.
pub struct SpendHost {
    pub consent: SpendConsent,
    /// A confirm is open (a second action waits for it instead of opening another).
    pub asking: bool,
    pub window: AnyWindowHandle,
}

impl SpendHost {
    pub(super) fn new(window: AnyWindowHandle) -> Self {
        Self { consent: SpendConsent::default(), asking: false, window }
    }
}

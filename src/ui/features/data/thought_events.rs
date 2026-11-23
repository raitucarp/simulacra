//! N16–N20: the interactions of the lens fan, the glossary, open questions, "Changed my
//! mind" and bilingual posts, carried to the shell as one [`DataEvent::Thought`](super::DataEvent).

/// One N16–N20 interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThoughtEvent {
    /// N16: the lens fan sheet.
    Lens(super::lens_sheet::LensEvent),
}

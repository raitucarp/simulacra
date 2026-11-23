//! N16–N20: the interactions of the lens fan, the glossary, open questions, "Changed my
//! mind" and bilingual posts, carried to the shell as one [`DataEvent::Thought`](super::DataEvent).

/// One N16–N20 interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThoughtEvent {
    /// N16: the lens fan sheet.
    Lens(super::lens_sheet::LensEvent),
    /// N17: defining terms.
    Glossary(super::glossary_sheet::GlossaryEvent),
    /// N18: open questions.
    Questions(super::questions_sheet::QuestionsEvent),
    /// N19: "Changed my mind".
    Mind(super::mind_sheet::MindEvent),
    /// N20: bilingual posts.
    Translate(super::translation_sheet::TranslateEvent),
}

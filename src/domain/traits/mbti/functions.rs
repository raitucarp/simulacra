//! Jungian cognitive-function stacks and descriptions per MBTI archetype.

use super::MbtiType;
use crate::domain::trait_option::trait_table;

trait_table! {
    /// Returns the 4-function Harold Grant / Jungian cognitive stack (Dominant · Auxiliary · Tertiary · Inferior).
    MbtiType::cognitive_functions {
        Intj => "Ni · Te · Fi · Se",
        Intp => "Ti · Ne · Si · Fe",
        Entj => "Te · Ni · Se · Fi",
        Entp => "Ne · Ti · Fe · Si",
        Infj => "Ni · Fe · Ti · Se",
        Infp => "Fi · Ne · Si · Te",
        Enfj => "Fe · Ni · Se · Ti",
        Enfp => "Ne · Fi · Te · Si",
        Istj => "Si · Te · Fi · Ne",
        Isfj => "Si · Fe · Ti · Ne",
        Estj => "Te · Si · Ne · Fi",
        Esfj => "Fe · Si · Ne · Ti",
        Istp => "Ti · Se · Ni · Fe",
        Isfp => "Fi · Se · Ni · Te",
        Estp => "Se · Ti · Fe · Ni",
        Esfp => "Se · Fi · Te · Ni",
    }
}

trait_table! {
    MbtiType::cognitive_description {
        Intj => "Introverted Intuition (Ni) dominant with Extraverted Thinking (Te): synthesizes long-range patterns into rigorous systems.",
        Intp => "Introverted Thinking (Ti) dominant with Extraverted Intuition (Ne): deconstructs axioms and explores conceptual possibilities.",
        Entj => "Extraverted Thinking (Te) dominant with Introverted Intuition (Ni): marshals evidence and structures decisive frameworks.",
        Entp => "Extraverted Intuition (Ne) dominant with Introverted Thinking (Ti): interrogates premises via Socratic counter-examples.",
        Infj => "Introverted Intuition (Ni) dominant with Extraverted Feeling (Fe): perceives moral-existential depth and human subtext.",
        Infp => "Introverted Feeling (Fi) dominant with Extraverted Intuition (Ne): guided by authentic inner ethics and poetic imagination.",
        Enfj => "Extraverted Feeling (Fe) dominant with Introverted Intuition (Ni): fosters collective insight and empathetic mentorship.",
        Enfp => "Extraverted Intuition (Ne) dominant with Introverted Feeling (Fi): radiates associative curiosity and existential warmth.",
        Istj => "Introverted Sensing (Si) dominant with Extraverted Thinking (Te): grounds judgment in verified history and empirical precedent.",
        Isfj => "Introverted Sensing (Si) dominant with Extraverted Feeling (Fe): preserves human memory, care, and continuity.",
        Estj => "Extraverted Thinking (Te) dominant with Introverted Sensing (Si): enforces clear standards and practical accountability.",
        Esfj => "Extraverted Feeling (Fe) dominant with Introverted Sensing (Si): nurtures community cohesion and shared norms.",
        Istp => "Introverted Thinking (Ti) dominant with Extraverted Sensing (Se): masters mechanical sympathy and direct first-principles craft.",
        Isfp => "Introverted Feeling (Fi) dominant with Extraverted Sensing (Se): expresses quiet aesthetic sensitivity and lived presence.",
        Estp => "Extraverted Sensing (Se) dominant with Introverted Thinking (Ti): thrives on immediate tactical reality and kinetic wit.",
        Esfp => "Extraverted Sensing (Se) dominant with Introverted Feeling (Fi): celebrates vivid sensory experience and spontaneous connection.",
    }
}

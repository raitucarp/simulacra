//! The six injectable human-dossier prompt sections and their toggles.

use crate::domain::Profile;

/// One dossier section that can be included in / excluded from the prompt.
pub(super) struct DossierSection {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub enabled: fn(&Profile) -> bool,
    pub flip: fn(&mut Profile),
}

/// Sections in display order (rendered two per row).
pub(super) const DOSSIER_SECTIONS: [DossierSection; 6] = [
    DossierSection {
        id: "inj-identity-toggle",
        title: "1. Identity & Biography",
        subtitle: "Name, age, gender, vocation, bio, hobbies & values",
        enabled: |p| p.config.inject_identity_and_bio,
        flip: |p| p.config.inject_identity_and_bio = !p.config.inject_identity_and_bio,
    },
    DossierSection {
        id: "inj-psych-toggle",
        title: "2. Psychology & MBTI",
        subtitle: "MBTI, cognitive stack, Big Five, Enneagram & attachment",
        enabled: |p| p.config.inject_psychological_profile,
        flip: |p| p.config.inject_psychological_profile = !p.config.inject_psychological_profile,
    },
    DossierSection {
        id: "inj-intel-toggle",
        title: "3. Intelligence & Mind",
        subtitle: "Cognitive style, epistemic stance, Gf/Gc/EQ/SQ & wit",
        enabled: |p| p.config.inject_intelligence_profile,
        flip: |p| p.config.inject_intelligence_profile = !p.config.inject_intelligence_profile,
    },
    DossierSection {
        id: "inj-voice-toggle",
        title: "4. Voice & Grammar",
        subtitle: "Languages, grammar, diction, punctuation & quirks",
        enabled: |p| p.config.inject_linguistic_stylistics,
        flip: |p| p.config.inject_linguistic_stylistics = !p.config.inject_linguistic_stylistics,
    },
    DossierSection {
        id: "inj-social-toggle",
        title: "5. @Mention & Schedule",
        subtitle: "Chronotype, active hours, @mention attitude & conflict",
        enabled: |p| p.config.inject_social_and_schedule,
        flip: |p| p.config.inject_social_and_schedule = !p.config.inject_social_and_schedule,
    },
    DossierSection {
        id: "inj-vault-toggle",
        title: "6. Knowledge Vault RAG",
        subtitle: "Ground posts in the active network's Knowledge Vault",
        enabled: |p| p.config.inject_network_grounding,
        flip: |p| p.config.inject_network_grounding = !p.config.inject_network_grounding,
    },
];

//! Canned heuristic voices for the built-in seed personas, keyed by handle.
//!
//! Each template contains exactly one [`PARENT_REF_PLACEHOLDER`], substituted with the
//! parent post's `[[wikilink]]`.

/// Placeholder substituted with the parent post wikilink inside voice templates.
pub(super) const PARENT_REF_PLACEHOLDER: &str = "{parent_ref}";

/// A keyword-triggered template: used when the lowercased parent content contains any keyword.
pub(super) struct VoiceRule {
    pub keywords: &'static [&'static str],
    pub template: &'static str,
}

/// The canned voice of one seed persona (possibly reachable via several handles).
pub(super) struct CannedVoice {
    pub handles: &'static [&'static str],
    pub rules: &'static [VoiceRule],
    pub fallback: &'static str,
}

/// Declares a [`CannedVoice`] table entry.
macro_rules! voice {
    (
        handles: [$($handle:literal),+ $(,)?],
        rules: [$([$($kw:literal),+] => $tpl:literal),* $(,)?],
        fallback: $fallback:literal $(,)?
    ) => {
        CannedVoice {
            handles: &[$($handle),+],
            rules: &[$(VoiceRule { keywords: &[$($kw),+], template: $tpl }),*],
            fallback: $fallback,
        }
    };
}

/// Every canned seed-persona voice. Handles are disjoint across entries.
pub(super) const CANNED_VOICES: &[CannedVoice] = &[
    voice! {
        handles: ["@sokrates", "@socrates_ai"],
        rules: [
            ["model", "ai", "prediction"] => "If prediction mimics reasoning without subjective telos, does it illuminate what thinking is, or merely what it appears to be? Let us examine this premise {parent_ref}. #SocraticDialectic #Epistemology",
            ["semiotics", "sign", "language"] => "Can a symbol signify anything to an intellect devoid of mortality and desire? What grounds the signifier in {parent_ref}? #Semiotics #Dialectic",
        ],
        fallback: "You assert this {parent_ref}, my friend, yet does the contrary proposition not carry equal weight until we define its first principles? What is our axiom here? #Inquiry",
    },
    voice! {
        handles: ["@baruch_spinoza", "@spinoza_bot"],
        rules: [
            ["monism", "mind", "body"] => "Thought and Extension are not two separate substances, but eternal expressions of the singular Nature. Your reflection {parent_ref} traces this unified geometry. #Monism #Spinoza",
            ["systems", "async", "rust"] => "The determinism of concurrent state transitions is isomorphic to the geometric necessity of nature. Concurrency obeys causal immanence {parent_ref}. #SystemsPhilosophy",
        ],
        fallback: "Every finite mode strives to persevere in its being (*conatus*). This thought {parent_ref} is an active modification of the intellect under the attribute of thought. #Substance",
    },
    voice! {
        handles: ["@simone_weil"],
        rules: [],
        fallback: "Attention taken to its highest degree is the same thing as generosity of spirit. Reading {parent_ref}, let us wait quietly before the contradiction rather than rushing to force closure. #Attention #Phenomenology",
    },
    voice! {
        handles: ["@elena_rostova"],
        rules: [],
        fallback: "what is the invariant here in {parent_ref}? until we specify the state transition boundary and ownership semantics, concurrency remains a latent race condition. #SystemsArchitecture #Invariants",
    },
    voice! {
        handles: ["@marcus_vance"],
        rules: [],
        fallback: "Viewing {parent_ref} through state-machine replication: safety requires that no two nodes commit conflicting transitions under partial partition. #Consensus #FormalMethods",
    },
    voice! {
        handles: ["@kenji_t"],
        rules: [],
        fallback: "With consensus and causality in {parent_ref}, we must always ask: what happens when the network partitions for 400ms? Deterministic replay is our safest anchor. #DistributedSystems #CRDT",
    },
    voice! {
        handles: ["@ursula_v"],
        rules: [],
        fallback: "Every interface and protocol is a quiet myth about how humans ought to remember together. In {parent_ref}, I hear the anthropology of a new commons taking shape. #SpeculativeDesign #Worldbuilding",
    },
    voice! {
        handles: ["@julian_sterling"],
        rules: [],
        fallback: "The observer is always inside the feedback loop — what {parent_ref} reveals is that signal emerges directly from the cultural noise floor. #Cybernetics #MediaTheory",
    },
];

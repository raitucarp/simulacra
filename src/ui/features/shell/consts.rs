//! Named literals of the application shell: column sizes, element ids, placeholders,
//! the avatar dialog title and the curated identities offered by "new agent / persona".

/// Pixels of stream rows rendered beyond the viewport so scrolling never pops in.
pub(super) const STREAM_LIST_OVERDRAW: f32 = 600.0;

/// Width of the left sidebar in its collapsed icon-rail mode.
pub(super) const LEFT_SIDEBAR_COLLAPSED_WIDTH: f32 = 64.0;
/// Resize range of the collapsed left sidebar.
pub(super) const LEFT_SIDEBAR_COLLAPSED_MIN: f32 = 60.0;
pub(super) const LEFT_SIDEBAR_COLLAPSED_MAX: f32 = 68.0;
/// Width of the expanded left sidebar.
pub(super) const LEFT_SIDEBAR_EXPANDED_WIDTH: f32 = 240.0;
/// Resize range of the expanded left sidebar.
pub(super) const LEFT_SIDEBAR_EXPANDED_MIN: f32 = 180.0;
pub(super) const LEFT_SIDEBAR_EXPANDED_MAX: f32 = 380.0;
/// Width of the right sidebar.
pub(super) const RIGHT_SIDEBAR_WIDTH: f32 = 320.0;
/// Resize range of the right sidebar.
pub(super) const RIGHT_SIDEBAR_MIN: f32 = 260.0;
pub(super) const RIGHT_SIDEBAR_MAX: f32 = 460.0;

/// Element id of the three-column layout while the left sidebar is collapsed.
pub(super) const LAYOUT_ID_COLLAPSED: &str = "simulacra-main-layout-collapsed";
/// Element id of the three-column layout while the left sidebar is expanded.
pub(super) const LAYOUT_ID_EXPANDED: &str = "simulacra-main-layout-expanded";

/// Auto-grow row ranges `(min, max)` of the shell's text areas.
pub(super) const COMPOSER_ROWS: (usize, usize) = (1, 5);
pub(super) const DOC_CONTENT_ROWS: (usize, usize) = (3, 8);
pub(super) const EDITOR_LONG_ROWS: (usize, usize) = (3, 6);
pub(super) const EDITOR_SHORT_ROWS: (usize, usize) = (2, 5);

/// Placeholders of the LLM settings inputs.
pub(super) const LLM_API_KEY_PLACEHOLDER: &str = "Enter Gemini API Key (AIzaSy...)";
pub(super) const LLM_MODEL_PLACEHOLDER: &str = "Model Identifier (e.g. gemini-1.5-flash)";
pub(super) const LLM_ENDPOINT_PLACEHOLDER: &str = "http://localhost:8000/v1/chat/completions";

/// File dialog title of the agent editor avatar picker.
pub(super) const PICK_AVATAR_TITLE: &str = "Select Avatar Portrait Image";

/// Prefix that marks a profile handle.
pub(super) const HANDLE_PREFIX: char = '@';
/// Quote character stripped from pasted file paths (e.g. Windows "Copy as path").
pub(super) const PATH_QUOTE: char = '"';

/// `(id, name, handle, occupation)` of the editor's profile when the store has none.
pub(super) const FALLBACK_LAB_PROFILE: (&str, &str, &str, &str) =
    ("agent_default", "Sokrates", "@sokrates", "Philosopher");

/// A curated `(name, handle, role)` identity for a newly created profile.
pub(super) type CuratedIdentity = (&'static str, &'static str, &'static str);

/// Identities offered, in order, when creating a persona.
pub(super) const CURATED_PERSONAS: [CuratedIdentity; 4] = [
    ("Elena Vance", "@elena_vance", "Literary Essayist & Cultural Critic"),
    ("Reza Mahendra", "@reza_m", "Systems Architect & Phenomenologist"),
    ("Clara Lindqvist", "@clara_l", "Cognitive Scientist & Researcher"),
    ("Julian Thorne", "@julian_t", "Historian of Ideas"),
];
/// Persona identity used once the curated list is exhausted.
pub(super) const FALLBACK_PERSONA: CuratedIdentity =
    ("Nadia Prameswari", "@nadia_p", "Independent Writer");
/// Number of personas a network starts with before the curated list applies.
pub(super) const CURATED_PERSONA_OFFSET: usize = 2;

/// Identities offered, in order, when creating a network agent.
pub(super) const CURATED_AGENTS: [CuratedIdentity; 4] = [
    ("Michel de Montaigne", "@montaigne", "Humanist Essayist & Skeptic"),
    ("Ibn Khaldun", "@ibn_khaldun", "Historian & Social Theorist"),
    ("Susanne Langer", "@susanne_langer", "Philosopher of Mind & Art"),
    ("Tan Malaka", "@tan_malaka", "Materialist Dialectician & Educator"),
];
/// Agent identity used once the curated list is exhausted.
pub(super) const FALLBACK_AGENT: CuratedIdentity =
    ("Arthur Schopenhauer", "@schopenhauer", "Metaphysician & Aesthete");
/// Number of agents a network starts with before the curated list applies.
pub(super) const CURATED_AGENT_OFFSET: usize = 3;

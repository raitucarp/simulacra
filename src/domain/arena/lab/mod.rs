//! The thought-experiment lab: a scenario (premise, variables of two to four values, a
//! question), a grid run over every combination of values for each chosen agent with its own
//! budget preview, each agent's answer per cell with a one-line verdict, and a comparison of
//! where the agents differ and which variable flips their answers.
//!
//! - [`scenario`]: the scenario, validation and the grid's cells.
//! - [`record`]: a saved experiment and its answers.
//! - [`prompt`]: one cell's request and the reader of its answer.
//! - [`budget`]: the run's preview against today's AI budget.
//! - [`compare`]: disagreements, agreement between agents, variable flips.
//! - [`presets`]: classic experiments as editable starting points.
//! - [`summary`]: the summary post.

pub mod budget;
pub mod compare;
pub mod presets;
pub mod prompt;
pub mod record;
pub mod scenario;
pub mod summary;

pub use budget::{preview, RunPreview, MAX_LAB_REQUESTS};
pub use compare::{disagreements, flips, pairwise, PairAgreement, VariableFlips};
pub use presets::{preset_scenario, PresetKey};
pub use prompt::{cell_prompt, parse_cell_answer, CELL_ANSWER_TOKENS};
pub use record::{CellAnswer, Experiment, Stance};
pub use scenario::{cells, conditions, joined_values, parse_values, scenario_from, scenario_problem, Scenario, ScenarioProblem, Variable, VariableDraft, MAX_VALUES, MAX_VARIABLES, MIN_VALUES};

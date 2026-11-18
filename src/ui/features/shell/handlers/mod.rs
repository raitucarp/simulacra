//! `AppView` event handlers, grouped by responsibility. Each is a small method wired to a
//! UI callback by `super::binder::Binder`.

pub(super) mod appearance;
mod capture;
mod chrome;
mod commands;
mod composer_modal;
mod credentials;
mod import_flow;
mod import_sheet;
mod inspector;
mod lab;
mod navigation;
mod networks;
mod onboarding;
mod palette;
mod post_actions;
mod post_actions_context;
mod post_actions_output;
mod post_actions_run;
mod post_menu;
mod posts;
mod preferences;
mod review_cards;
mod review_events;
mod review_quiz;
mod settings_commands;
mod scheduler;
mod semantic_index;
mod semantic_pump;
mod semantic_search;
mod semantic_similar;
mod settings;
mod sidebar_nav;
mod simulation;
mod simulation_notices;
mod storage;
mod stream;

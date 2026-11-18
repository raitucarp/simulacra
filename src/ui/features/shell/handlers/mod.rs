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
mod lab;
mod navigation;
mod networks;
mod onboarding;
mod palette;
mod post_menu;
mod posts;
mod preferences;
mod review_events;
mod review_learn;
mod review_maps;
mod review_quiz;
mod review_sheet;
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

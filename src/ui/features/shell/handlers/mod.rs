//! `AppView` event handlers, grouped by responsibility. Each is a small method wired to a
//! UI callback by `super::binder::Binder`.

mod chrome;
mod composer_modal;
mod credentials;
mod lab;
mod navigation;
mod networks;
mod onboarding;
mod post_menu;
mod posts;
mod scheduler;
mod settings;
mod simulation;
mod simulation_notices;
mod storage;
mod stream;
mod vault_import;

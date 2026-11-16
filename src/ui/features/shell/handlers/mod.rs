//! `AppView` event handlers, grouped by responsibility. Each is a small method wired to a
//! UI callback by `super::binder::Binder`.

mod chrome;
mod composer_modal;
mod lab;
mod lab_profiles;
mod navigation;
mod posts;
mod scheduler;
mod settings;
mod simulation;
mod simulation_notices;
mod storage;
mod stream;

//! `AppView` event handlers, grouped by responsibility. Each is a small method wired to a
//! UI callback by `super::binder::Binder`.

pub(super) mod appearance;
mod arena;
mod arena_calls;
mod arena_debate;
mod arena_judge;
mod arena_lab;
mod arena_publish;
mod arena_spend; // sb P16
mod arena_sync;
mod beliefs;
mod beliefs_material;
mod capture;
mod chrome;
mod commands;
mod composer_modal;
mod composer_locked; // X3
mod composer_locked_edit; // X5
mod feeling_journal; // M6
mod credentials;
mod data; // sb F2: history, export, snapshots, collections and bookmarks
mod data_export;
mod data_history;
mod data_legend; // S9
mod data_margin; // N2
mod data_branch; // N6
mod data_quote; mod data_ideas; // N7, N6–N10
mod data_question; // N9
mod data_series; // N10
mod data_garden; // N3
mod feed_answer; // N4
mod data_depth; // N5
mod data_posts;
mod data_restore;
mod data_review; // S16
mod data_saved;
mod data_scheduled; // Y4
mod data_shelf;
mod data_snapshot;
mod entry_state; // S7
mod todos; // D6: ticking to-dos
mod image_ideas;
mod image_ideas_apply;
mod image_ideas_open;
mod image_ideas_run;
mod import_flow;
mod import_sheet;
mod insights;
mod journal;
mod journal_events;
mod inspector;
mod lab;
mod learning_data;
mod learning_events;
mod learning_locus;
mod learning_tutor;
mod learning_tutor_run;
mod media;
mod media_pump;
mod file_pump; // L7: scans of attached files
mod relink; // L11: relinking a moved file
mod navigation;
mod networks;
mod onboarding;
mod palette;
mod post_actions;
mod profile_posts; // W7
mod trigger_dialog; // A4
mod manual_runs; // A5
mod post_actions_context;
mod post_actions_output;
mod post_actions_run;
mod post_expiry; // Y7
mod post_lifecycle;
mod post_menu;
mod posts;
mod preferences;
mod reactions; // epic R
mod reactions_clap; // R7
mod review_events;
mod review_learn;
mod review_maps;
mod review_quiz;
mod review_sheet;
mod settings_commands;
mod scheduler;
mod symbol_downloads; // S13
pub(in crate::ui::features::shell) use symbol_downloads::DownloadJob;
mod semantic_index;
mod semantic_pump;
mod semantic_search;
mod post_search; // Q4
mod code_run; mod code_confirm; // P5
mod semantic_similar;
mod settings;
mod sidebar_nav;
mod simulation;
mod simulation_job;
mod simulation_notices;
mod spend; // sb P16: the budget path and ask before spending
pub(super) use spend::SpendResume;
mod spend_dialog;
mod storage;
mod stream;
mod thinking;
mod thinking_ai;
mod thinking_argument;
mod thinking_atomic;
mod thinking_credence;
mod thinking_insights;
mod thinking_tensions;
mod thinking_tensions_scan;
mod unlock; // X4: unlocking locked posts
mod unlock_backoff; // X7

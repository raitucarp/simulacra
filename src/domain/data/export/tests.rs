use super::*;
use crate::domain::data::saved::ItemRef;
use crate::domain::document::Collection;
use crate::domain::{NetworkId, PostId};
use jiff::Timestamp;
use serde_json::json;

const NET: &str = "net_main";

fn at(day: i8) -> Timestamp {
    crate::time::utc(2026, 11, day, 12, 0, 0)
}

fn post(id: &str, author: &str, text: &str, day: i8, reply_to: Option<&str>) -> Post {
    let mut post = Post::create(id, NET, author, text).unwrap();
    post.created_at = at(day);
    post.reply_to_id = reply_to.map(PostId::new);
    post
}

fn source() -> ExportSource {
    let me = Profile::new_primary("user_main", "Ada", "@ada");
    let mut socrates = Profile::new_agent("agent_s", "Socrates", "@socrates", "Philosopher");
    socrates.config.engine.request = crate::domain::engine::RequestMode::Custom(Default::default());
    let posts = vec![
        post("p1", "user_main", "Is the mind a [[mirror]]? #mind", 2, None),
        post("p2", "agent_s", "Or a lamp, Ada?", 3, Some("p1")),
        post("p3", "user_main", "A lamp, then.", 9, Some("p2")),
        post("p4", "agent_s", "Unrelated musing, with \"quotes\".", 5, None),
    ];
    let mut doc = GroundingDocument::new("doc_ethics", NET, "Ethics", "Deus sive natura.");
    doc.created_at = at(4);
    doc.record.collections = vec![CollectionId::new("coll_a")];
    doc.record.provenance.original = "abcd.epub".into();
    let library = Library { subjects: Vec::new(), collections: vec![Collection { id: CollectionId::new("coll_a"), network_id: NetworkId::new(NET), name: "Shelf".into(), description: String::new() }] };
    let entries = vec![CollectionEntry { collection: CollectionId::new("coll_a"), item: ItemRef::Post(PostId::new("p4")), position: 0, note: String::new(), added_at: at(6) }];
    let authors = [me.clone(), socrates.clone()].into_iter().map(|p| (p.id.clone(), p)).collect();
    ExportSource {
        network_name: "Main".into(), posts, authors, profiles: vec![me, socrates], documents: vec![doc], library, review: Vec::new(), entries,
        settings: vec![("preferences.general".into(), "{\"language\":\"en\"}".into())], journal: None, reactions: Vec::new(),
        media_dir: PathBuf::from("media"), exported_at: at(10), app_version: "0.1.0".into(),
    }
}

fn options(scope: ExportScope) -> ExportOptions {
    ExportOptions { scope, include_originals: false, include_journal: false }
}

fn paths(files: &[BundleFile]) -> Vec<&str> {
    files.iter().map(|f| f.path.as_str()).collect()
}

fn text_of<'a>(files: &'a [BundleFile], prefix: &str) -> &'a str {
    files.iter().find(|f| f.path.starts_with(prefix)).and_then(|f| match &f.body { FileBody::Text(t) => Some(t.as_str()), FileBody::Copy(_) => None }).unwrap_or_default()
}

#[test]
fn everything_groups_posts_by_thread() {
    let source = source();
    let plan = plan_export(&source, &options(ExportScope::Everything));
    assert_eq!((plan.counts.threads, plan.counts.posts, plan.counts.documents, plan.counts.agents, plan.counts.settings), (2, 4, 1, 2, 1));
    assert_eq!(plan.threads[0].posts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p1", "p2", "p3"]);
}

#[test]
fn persona_scope_keeps_only_its_posts_in_their_threads() {
    let source = source();
    let plan = plan_export(&source, &options(ExportScope::Persona(ProfileId::new("user_main"))));
    assert_eq!(plan.threads.len(), 1);
    assert_eq!(plan.threads[0].root.as_str(), "p1");
    assert_eq!(plan.threads[0].posts.len(), 2);
    assert_eq!(plan.counts.settings, 0, "settings travel with everything only");
    assert_eq!(plan.agents.iter().map(|p| p.handle.as_str()).collect::<Vec<_>>(), vec!["@ada"]);
}

#[test]
fn collection_scope_takes_its_posts_and_filed_documents() {
    let source = source();
    let plan = plan_export(&source, &options(ExportScope::Collection(CollectionId::new("coll_a"))));
    assert_eq!((plan.counts.posts, plan.counts.documents), (1, 1));
    assert_eq!(plan.threads[0].root.as_str(), "p4");
}

#[test]
fn date_range_is_inclusive() {
    let source = source();
    let day = |d: i8| crate::time::local_date(at(d));
    let plan = plan_export(&source, &options(ExportScope::DateRange { from: day(3), to: day(5) }));
    assert_eq!(plan.counts.posts, 2);
    assert_eq!(plan.counts.documents, 1);
}

#[test]
fn thread_markdown_has_front_matter_and_keeps_wikilinks() {
    let source = source();
    let plan = plan_export(&source, &options(ExportScope::Everything));
    let files = bundle_files(&plan, &source);
    let thread = text_of(&files, "threads/2026-11-02-is-the-mind-a-mirror-mind");
    assert!(thread.starts_with("---\ntitle: \"Is the mind a [[mirror]]? #mind\"\n"), "{thread}");
    assert!(thread.contains("tags: [\"#mind\"]"));
    assert!(thread.contains("authors: [\"@ada\",\"@socrates\"]
"), "{thread}");
    assert!(thread.contains("### Socrates (@socrates) · 2026-11-03 12:00 UTC"));
    assert!(thread.contains("[[mirror]]"));
}

/// A thread exported as Markdown and imported again (paragraph units) gives back every
/// post's text with its marks, code block and language tag.
#[test]
fn marks_survive_a_markdown_round_trip() {
    use crate::import::{parse_document, segment_document, Format, Granularity};
    let bodies = [
        "**Bold**, *italic*, __underlined__, `code` and ||a spoiler|| with #Tag and [[mirror]].",
        "```rust\nfn main() {\n    let x = 1;\n}\n```",
        "***Both*** and **nested *italic***.",
    ];
    let replies_to = [None, Some("m0"), Some("m1")];
    let mut source = source();
    source.posts = bodies.iter().zip(replies_to).enumerate().map(|(i, (text, parent))| post(&format!("m{i}"), "user_main", text, 2 + i as i8, parent)).collect();
    let plan = plan_export(&source, &options(ExportScope::Everything));
    let files = bundle_files(&plan, &source);
    let markdown = files.iter().find_map(|f| match &f.body { FileBody::Text(t) if f.path.starts_with("threads/") => Some(t.clone()), _ => None }).expect("thread file");
    let doc = parse_document(Format::Markdown, markdown.as_bytes(), "thread.md");
    let units: Vec<String> = segment_document(&doc, Granularity::Paragraph).into_iter().map(|unit| unit.text).collect();
    for body in bodies {
        assert!(units.iter().any(|unit| unit == body), "lost {body:?} in {units:#?}");
    }
    assert!(!units.iter().any(|unit| unit.trim() == "---"), "post separators are not posts");
}

#[test]
fn bundle_lists_manifest_threads_documents_agents_and_settings() {
    let source = source();
    let plan = plan_export(&source, &ExportOptions { include_originals: true, ..options(ExportScope::Everything) });
    let files = bundle_files(&plan, &source);
    let all = paths(&files);
    assert_eq!(all[0], MANIFEST_PATH);
    assert!(all.iter().any(|p| p.starts_with("documents/ethics-") && p.ends_with(".md")));
    assert!(all.contains(&"documents/originals/abcd.epub"));
    assert!(all.iter().any(|p| p.starts_with("agents/socrates-")));
    assert!(all.contains(&"settings.json"));
    assert!(!all.iter().any(|p| p.starts_with("journal/")));
    let manifest: serde_json::Value = serde_json::from_str(text_of(&files, MANIFEST_PATH)).unwrap();
    assert_eq!(manifest["format"], json!(BUNDLE_FORMAT));
    assert_eq!(manifest["counts"]["posts"], json!(4));
    assert_eq!(manifest["includes_private_journal"], json!(false));
}

#[test]
fn journal_is_left_out_unless_ticked() {
    let mut source = source();
    let day = crate::time::local_date(at(3));
    let entry = crate::domain::journal::JournalEntry::new(crate::domain::journal::RecordId::new("j1"), day, at(3), "private thought");
    source.journal = Some(crate::domain::journal::JournalBook { entries: vec![entry], ..Default::default() });
    let off = bundle_files(&plan_export(&source, &options(ExportScope::Everything)), &source);
    assert!(!off.iter().any(|f| f.path.starts_with("journal/")));
    let on_plan = plan_export(&source, &ExportOptions { include_journal: true, ..options(ExportScope::Everything) });
    assert_eq!(on_plan.counts.journal_records, 1);
    assert!(text_of(&bundle_files(&on_plan, &source), "journal/").contains("private thought"));
}

#[test]
fn settings_allow_list_holds_no_secret() {
    use crate::storage::SettingKey as K;
    for forbidden in [K::Credentials, K::LlmApiKey, K::LlmProvider, K::LlmEndpoint, K::LlmModel, K::EmbeddingUsage, K::ForgeAutosave, K::LegacyThemeTokens] {
        assert!(!SETTINGS_ALLOW_LIST.contains(&forbidden), "{forbidden:?} must stay out");
    }
    for key in SETTINGS_ALLOW_LIST {
        let name = key.as_str().to_lowercase();
        assert!(!["key", "token", "secret", "credential", "password"].iter().any(|m| name.contains(m)), "{name}");
    }
}

#[test]
fn agent_secrets_are_redacted() {
    let mut agent = Profile::new_agent("agent_x", "X", "@x", "Tester");
    let mut request = crate::domain::engine::CustomRequest { url: "https://example.test/v1?key=SECRET1".into(), credential: Some("cred_main".into()), ..Default::default() };
    request.headers[0].value = "Bearer SECRET2".into();
    agent.config.engine.request = crate::domain::engine::RequestMode::Custom(request);
    let json = redacted_profile(&agent).to_string();
    assert!(!json.contains("SECRET1") && !json.contains("SECRET2") && !json.contains("cred_main"), "{json}");
    assert!(json.contains("https://example.test/v1") && json.contains("Authorization"));
}

#[test]
fn csv_fields_are_quoted_when_needed() {
    assert_eq!(csv_field("plain"), "plain");
    assert_eq!(csv_field("a,b"), "\"a,b\"");
    assert_eq!(csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
    assert_eq!(csv_field("two\nlines"), "\"two\nlines\"");
    assert!(review_csv(&[]).starts_with("id,type,anchor,written,tags,created,due,interval_days,reps,lapses,suspended\r\n"));
}

#[test]
fn slugs_and_titles_are_tidy() {
    assert_eq!(slug("Is the mind a [[mirror]]?"), "is-the-mind-a-mirror");
    assert_eq!(slug("日本語"), "untitled");
    assert_eq!(thread_title("\n  First line\nsecond"), "First line");
    assert!(thread_title(&"word ".repeat(40)).ends_with('…'));
}

#[test]
fn reply_cycles_do_not_hang_the_root_search() {
    let a = post("a", "u", "a", 1, Some("b"));
    let b = post("b", "u", "b", 1, Some("a"));
    let by_id: std::collections::HashMap<&str, &Post> = [("a", &a), ("b", &b)].into_iter().collect();
    let root = thread_root(&a, &by_id);
    assert!(root.id.as_str() == "a" || root.id.as_str() == "b");
}

#[test]
fn reactions_of_exported_posts_round_trip_through_the_bundle() {
    use crate::domain::reaction::{Reaction, CLAP};
    let row = |post: &str, profile: &str, emoji: &str, count: u32| Reaction { post_id: PostId::new(post), profile_id: ProfileId::new(profile), emoji: emoji.into(), count, updated_at: at(7) };
    let mut source = source();
    source.reactions = vec![row("p1", "agent_s", "\u{1F525}", 1), row("p1", "user_main", CLAP, 12), row("p4", "user_main", "\u{1F914}", 1)];
    let everything = plan_export(&source, &options(ExportScope::Everything));
    assert_eq!(everything.counts.reactions, 3);
    let files = bundle_files(&everything, &source);
    let text = text_of(&files, REACTIONS_PATH);
    assert!(text.contains("@socrates"), "rows name who reacted");
    assert_eq!(read_reactions(text).expect("reads back"), source.reactions);
    assert!(text_of(&files, MANIFEST_PATH).contains("\"reactions\": 3"));

    let persona = plan_export(&source, &options(ExportScope::Persona(ProfileId::new("user_main"))));
    assert_eq!(persona.reactions, source.reactions[..2].to_vec(), "only reactions to exported posts travel");
    let mut empty = self::source();
    empty.reactions.clear();
    assert!(!paths(&bundle_files(&plan_export(&empty, &options(ExportScope::Everything)), &empty)).contains(&REACTIONS_PATH));
    assert!(read_reactions("{}").is_err());
    assert_eq!(read_reactions(r#"[{"post_id":"p1","profile_id":"u","emoji":"lol","count":1,"updated_at":"2026-11-07T12:00:00Z"}]"#), Ok(Vec::new()), "text that is not an emoji is left out");
}

/// X6: a locked post leaves only its seal in the bundle, and the exported thread imports back
/// as a locked post that opens with the same password.
#[test]
fn a_locked_post_travels_as_ciphertext_and_imports_locked() {
    use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
    use crate::import::plan::{build_posts, plan_posts, SourceInfo};
    use crate::import::{parse_document, segment_document, Format, Granularity};
    let password = LockPassword::new("seven tidy lanterns");
    let mut locked = post("p5", "user_main", "Tamarind lighthouse ledger", 6, Some("p4"));
    locked.lock_with(&password, crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    let mut source = source();
    source.posts.push(locked.clone());
    let files = bundle_files(&plan_export(&source, &options(ExportScope::Everything)), &source);
    let texts: Vec<&str> = files.iter().filter_map(|f| match &f.body { FileBody::Text(t) => Some(t.as_str()), FileBody::Copy(_) => None }).collect();
    assert!(texts.iter().all(|text| !text.contains("Tamarind") && !text.contains("lighthouse")), "no plaintext in any file");
    let thread = texts.iter().find(|text| text.contains("post-lock")).expect("the thread with the seal");
    let doc = parse_document(Format::Markdown, thread.as_bytes(), "thread.md");
    let units = segment_document(&doc, Granularity::Paragraph);
    let info = SourceInfo { document_id: "doc".into(), title: "Thread".into(), chapter_titles: Vec::new() };
    let mut next = 0;
    let posts = build_posts(&plan_posts(&units, &info, crate::domain::MAX_POST_CHARS), &ProfileId::new("user_main"), &NetworkId::new(NET), at(12), crate::domain::post::PostOrigin::Import, || {
        next += 1;
        PostId::new(format!("imported_{next}"))
    })
    .expect("imported");
    let back: Vec<&Post> = posts.iter().filter(|p| p.is_locked()).collect();
    assert_eq!(back.len(), 1, "{posts:#?}");
    assert_eq!((back[0].content.as_str(), &back[0].extras.lock), (LOCKED_PLACEHOLDER, &locked.extras.lock));
    assert_eq!(back[0].unlock(&password).expect("opens after the round trip").as_str(), "Tamarind lighthouse ledger");
    assert!(posts.iter().any(|p| p.content == "Unrelated musing, with \"quotes\"."), "the open posts import as before");
}

/// N7, N10: a pinned quote and a series part travel with the post, never with a locked one.
#[test]
fn pinned_quotes_and_series_travel_but_never_past_a_lock() {
    use crate::domain::post::lock::LockPassword;
    use crate::domain::post::pinned::PinnedQuote;
    use crate::domain::post::series::{series_id, SeriesEntry};
    let mut source = source();
    source.posts[0].extras.pinned = PinnedQuote::of("doc_ethics", "Ethics", "Deus sive natura.", 0..17).map(Box::new);
    source.posts[0].extras.series = Some(Box::new(SeriesEntry { id: series_id("Notes"), title: "Notes".into(), number: 3, goal: None }));
    let mut locked = post("p6", "user_main", "Tamarind lighthouse ledger", 7, None);
    locked.extras.pinned = PinnedQuote::of("doc_x", "Quetzal source", "Orchard confession.", 0..19).map(Box::new);
    locked.extras.series = Some(Box::new(SeriesEntry { id: series_id("Hidden"), title: "Quetzal diary".into(), number: 1, goal: None }));
    locked.lock_with(&LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    source.posts.push(locked);
    let files = bundle_files(&plan_export(&source, &options(ExportScope::Everything)), &source);
    let all: String = files.iter().filter_map(|f| match &f.body { FileBody::Text(t) => Some(t.as_str()), FileBody::Copy(_) => None }).collect();
    assert!(all.contains("> Deus sive natura.") && all.contains("Ethics") && all.contains("Notes #3"), "the open post keeps them");
    assert!(!all.contains("Quetzal") && !all.contains("Orchard"), "a locked post travels as its seal only");
}

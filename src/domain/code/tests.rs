//! Runtimes and their version probes (P1).

use super::*;

#[test]
fn versions_are_read_from_what_each_probe_prints() {
    assert_eq!(parse_version("v20.11.1\n").as_deref(), Some("20.11.1"));
    assert_eq!(parse_version("Python 3.12.2").as_deref(), Some("3.12.2"));
    assert_eq!(parse_version("go version go1.22.0 windows/amd64").as_deref(), Some("1.22.0"));
    assert_eq!(parse_version("deno 1.41.0 (release, x86_64)\nv8 12.1\n").as_deref(), Some("1.41.0"));
    assert_eq!(parse_version("Lua 5.4.6  Copyright (C) 1994-2023 Lua.org, PUC-Rio").as_deref(), Some("5.4.6"));
    assert_eq!(parse_version("PHP 8.3.4 (cli) (built: Mar 12 2024)").as_deref(), Some("8.3.4"));
    assert_eq!(parse_version("Python was not found; run without arguments to install"), None);
    assert_eq!(parse_version(""), None);
}

#[test]
fn every_runtime_has_a_label_and_a_version_probe() {
    for runtime in Runtime::ALL {
        let probe = runtime.probe();
        assert!(!probe.executables.is_empty() && !probe.args.is_empty(), "{runtime:?}");
        assert!(probe.args.iter().all(|arg| arg.contains("version") || *arg == "-v"), "probes only ask the version");
        assert!(!runtime.label().is_empty());
        assert_eq!(Runtime::from_key(runtime.key()), Some(*runtime));
    }
}

#[test]
fn the_cache_round_trips_and_reads_unknown_rows_as_never_detected() {
    let cache = RuntimeCache { found: vec![DetectedRuntime { runtime: Runtime::Lua, path: "/usr/bin/lua".into(), version: "5.4.6".into() }], probed_at: Some(jiff::Timestamp::UNIX_EPOCH) };
    let restored = RuntimeCache::from_json(&cache.to_json());
    assert_eq!(restored, cache);
    assert_eq!(restored.get(Runtime::Lua).map(|found| found.version.as_str()), Some("5.4.6"));
    assert!(restored.get(Runtime::Go).is_none());
    assert_eq!(RuntimeCache::from_json(r#"{"found":[{"runtime":"cobol","path":"x","version":"1"}]}"#), RuntimeCache::default());
}

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

// P3: the language → command table

#[test]
fn language_tags_pick_their_runtime() {
    let cases = [("python", Runtime::Python), ("Py", Runtime::Python), ("js", Runtime::Node), ("javascript", Runtime::Node), ("ts", Runtime::Deno), ("lua", Runtime::Lua), ("go", Runtime::Go), ("rb", Runtime::Ruby), ("php", Runtime::Php)];
    for (tag, runtime) in cases {
        assert_eq!(runtime_for(tag), Some(runtime), "{tag}");
    }
    for tag in ["", "rust", "sh", "bash", "powershell", "cmd", "text"] {
        assert_eq!(runtime_for(tag), None, "{tag} never runs");
    }
}

#[test]
fn every_runtime_runs_its_file_with_a_fixed_command_line() {
    for runtime in Runtime::ALL {
        let run = runtime.invocation();
        assert_eq!(run.args.last().map(String::as_str), Some(run.file_name), "{runtime:?} runs the code file last");
        assert!(run.file_name.starts_with("main.") && crate::runner::folder::is_plain_name(run.file_name));
        assert!(run.env.iter().any(|(name, value)| name == "NO_COLOR" && value == "1"));
        assert!(run.env.iter().all(|(_, value)| !value.contains('{') || value == "{dir}" || value == "{cache}"), "{runtime:?}");
    }
    assert_eq!(&Runtime::Python.invocation().args[..2], ["-I", "-B"], "isolated, no bytecode files");
    assert_eq!(Runtime::Go.invocation().args, ["run", "main.go"]);
    assert!(Runtime::Go.invocation().env.contains(&("GOPROXY".into(), "off".into())), "no module downloads");
    assert_eq!(Runtime::Php.invocation().args[0], "-n", "no php.ini");
}

// P6: saved runs

#[test]
fn saved_output_is_cut_and_each_block_keeps_its_latest_run() {
    let long = "x".repeat(SAVED_OUTPUT_CHARS + 10);
    let (kept, cut) = kept_output(&long);
    assert_eq!((kept.chars().count(), cut), (SAVED_OUTPUT_CHARS, true));
    assert_eq!(kept_output("ok\n"), ("ok\n".to_string(), false));
    let run = |block: usize, output: &str| CodeRun { block, output: output.into(), ..CodeRun::default() };
    let runs = with_run(with_run(with_run(Vec::new(), run(2, "b")), run(0, "a")), run(2, "c"));
    assert_eq!(runs.iter().map(|r| (r.block, r.output.as_str())).collect::<Vec<_>>(), vec![(0, "a"), (2, "c")]);
}

#[test]
fn a_run_reads_back_leniently() {
    let run = CodeRun { block: 1, runtime: "go".into(), version: "1.22.0".into(), end: RunEnd::TimedOut, duration_ms: 10_000, ..CodeRun::default() };
    let json = serde_json::to_string(&run).expect("json");
    assert_eq!(serde_json::from_str::<CodeRun>(&json).expect("back"), run);
    let newer: CodeRun = serde_json::from_str(r#"{"block":0,"end":"exploded","extra":1}"#).expect("lenient");
    assert_eq!(newer.end, RunEnd::Failed, "an end this version does not know");
    let extras = crate::domain::post::PostExtras::default();
    assert!(!serde_json::to_string(&extras).expect("json").contains("code_runs"), "no runs, nothing stored");
}

//! The command table against the runtimes installed here (P3): a trivial snippet each.
//! Runtimes that are not installed are skipped, with a note.

use crate::domain::code::Runtime;
use crate::runner::{detect, run_code, spec_for, End, Limits, CACHE_PLACEHOLDER, DIR_PLACEHOLDER};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// Roomy: the first Go run builds its standard library into the cache.
const LIMITS: Limits = Limits { timeout: Duration::from_secs(180), output_cap: 64 * 1024 };

fn snippet(runtime: Runtime) -> &'static str {
    match runtime {
        Runtime::Python => "print('hello from', 6 * 7)",
        Runtime::Node | Runtime::Deno => "console.log('hello from', 6 * 7)",
        Runtime::Lua => "print('hello from', 6 * 7)",
        Runtime::Go => "package main\n\nimport \"fmt\"\n\nfunc main() { fmt.Println(\"hello from\", 6*7) }\n",
        Runtime::Ruby => "puts \"hello from #{6 * 7}\"",
        Runtime::Php => "<?php echo 'hello from ', 6 * 7, PHP_EOL;",
    }
}

#[test]
fn the_placeholders_are_the_ones_the_table_uses() {
    let used: Vec<String> = Runtime::ALL.iter().flat_map(|runtime| runtime.invocation().env).map(|(_, value)| value).collect();
    assert!(used.iter().any(|value| value == DIR_PLACEHOLDER) && used.iter().any(|value| value == CACHE_PLACEHOLDER));
}

#[test]
fn installed_runtimes_run_a_trivial_snippet() {
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    for runtime in Runtime::ALL {
        let Some(found) = detect(*runtime, &path_var) else {
            eprintln!("skipped: {} is not installed", runtime.label());
            continue;
        };
        let spec = spec_for(&found, snippet(*runtime), LIMITS).expect("a runtime executable");
        let ran = run_code(&spec, &AtomicBool::new(false)).expect("ran");
        assert_eq!((ran.end, ran.exit_code), (End::Exited, Some(0)), "{}: {}", runtime.label(), ran.output);
        assert!(ran.output.contains("hello from") && ran.output.contains("42"), "{}: {}", runtime.label(), ran.output);
    }
}

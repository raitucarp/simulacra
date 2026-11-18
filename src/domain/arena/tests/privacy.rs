//! The privacy rule as a test: no code of the Arena or of belief memory touches the private
//! journal, mood, focus or capsule records, so none of them can reach an agent's prompt.

use std::fs;
use std::path::{Path, PathBuf};

/// Every module that builds or sends Arena and belief prompts.
const ROOTS: &[&str] = &["src/domain/arena", "src/domain/beliefs", "src/agent/arena", "src/ui/features/arena", "src/ui/features/beliefs"];
/// Shell files of the Arena and of belief updates.
const SHELL_DIR: &str = "src/ui/features/shell/handlers";
const SHELL_PREFIXES: &[&str] = &["arena", "beliefs"];
const PRIVATE_WORDS: &[&str] = &["journal", "mood_checkin", "focus_session", "time_capsule", "capsule"];

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for path in entries.flatten().map(|entry| entry.path()) {
        if path.is_dir() && path.file_name().is_none_or(|name| name != "tests") {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") && path.file_name().is_none_or(|name| name != "tests.rs") {
            out.push(path);
        }
    }
}

fn sources() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    ROOTS.iter().for_each(|dir| rust_files(&root.join(dir), &mut files));
    let shell = fs::read_dir(root.join(SHELL_DIR)).map(|entries| entries.flatten().map(|entry| entry.path()).collect::<Vec<_>>()).unwrap_or_default();
    files.extend(shell.into_iter().filter(|path| path.file_name().and_then(|name| name.to_str()).is_some_and(|name| SHELL_PREFIXES.iter().any(|prefix| name.starts_with(prefix)))));
    files
}

#[test]
fn arena_and_belief_code_never_touches_private_records() {
    let files = sources();
    assert!(files.len() > 10, "the scan found the Arena's sources");
    for file in files {
        let text = fs::read_to_string(&file).expect("readable source");
        for (number, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap_or_default().to_lowercase();
            if let Some(word) = PRIVATE_WORDS.iter().find(|word| code.contains(*word)) {
                panic!("{}:{} uses `{word}`: private records must never reach agents", file.display(), number + 1);
            }
        }
    }
}

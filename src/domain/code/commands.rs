//! Which runtime a code block runs in, and how (P3): the block's language tag picks the
//! runtime; each runtime has one fixed command line for a code file, the arguments as a list
//! (`python -I`, `node`, `deno run`, `lua`, `go run`, `ruby`, `php -n`), and the few variables
//! it needs on top of the cleared environment.

use super::runtime::Runtime;

/// In a variable's value, the run's own folder (see `runner::DIR_PLACEHOLDER`).
const DIR: &str = "{dir}";
/// In a variable's value, the runtime's cache kept between runs (see `runner::CACHE_PLACEHOLDER`).
const CACHE: &str = "{cache}";

/// How a runtime runs a code file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    /// The file the code is written to, in the run's folder.
    pub file_name: &'static str,
    /// The arguments after the executable; the file is named relative to the run's folder.
    pub args: Vec<String>,
    /// Variables on top of the cleared environment (`{dir}`, `{cache}` filled in by the runner).
    pub env: Vec<(String, String)>,
}

/// The runtime a code block tagged `lang` runs in; `None` for every other language.
pub fn runtime_for(lang: &str) -> Option<Runtime> {
    match lang.trim().to_ascii_lowercase().as_str() {
        "python" | "py" | "python3" => Some(Runtime::Python),
        "javascript" | "js" | "node" | "nodejs" => Some(Runtime::Node),
        "typescript" | "ts" | "deno" => Some(Runtime::Deno),
        "lua" => Some(Runtime::Lua),
        "go" | "golang" => Some(Runtime::Go),
        "ruby" | "rb" => Some(Runtime::Ruby),
        "php" => Some(Runtime::Php),
        _ => None,
    }
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

fn vars(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items.iter().map(|(name, value)| (name.to_string(), value.to_string())).collect()
}

/// No colour codes in captured output, whatever the runtime.
const NO_COLOR: (&str, &str) = ("NO_COLOR", "1");

impl Runtime {
    /// The fixed command line for a code file.
    pub fn invocation(self) -> Invocation {
        let (file_name, args, env): (&'static str, &[&str], Vec<(&str, &str)>) = match self {
            // -I: isolated (no user site, no PYTHON* variables), -B: no bytecode files.
            Self::Python => ("main.py", &["-I", "-B", "main.py"], vec![("PYTHONIOENCODING", "utf-8"), ("PYTHONUTF8", "1")]),
            Self::Node => ("main.js", &["main.js"], vec![]),
            // Deno grants no permission unless asked; --no-prompt keeps it from asking.
            Self::Deno => ("main.ts", &["run", "--no-prompt", "main.ts"], vec![("DENO_DIR", CACHE), ("DENO_NO_UPDATE_CHECK", "1")]),
            Self::Lua => ("main.lua", &["main.lua"], vec![]),
            // A single file outside any module; no toolchain or module downloads.
            Self::Go => ("main.go", &["run", "main.go"], vec![("GOCACHE", CACHE), ("GOPATH", DIR), ("GOENV", "off"), ("GOTOOLCHAIN", "local"), ("GOPROXY", "off")]),
            Self::Ruby => ("main.rb", &["--disable-gems", "main.rb"], vec![]),
            // -n: no php.ini.
            Self::Php => ("main.php", &["-n", "main.php"], vec![]),
        };
        let env = vars(&[NO_COLOR]).into_iter().chain(vars(&env)).collect();
        Invocation { file_name, args: strings(args), env }
    }
}

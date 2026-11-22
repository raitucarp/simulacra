//! The runtimes a code block may run in, and how each is asked its version (P1). Detection
//! only ever runs these probes: the executable with its version flag, nothing else.

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// A language runtime. Keys are stored (detection cache, saved runs): never rename one.
    Runtime {
        Node => "node",
        Deno => "deno",
        Python => "python",
        Lua => "lua",
        Go => "go",
        Ruby => "ruby",
        Php => "php",
    }
}

/// How a runtime is found and asked its version: the executable names tried on the search
/// path, in order, and the arguments of the version probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Probe {
    pub executables: &'static [&'static str],
    pub args: &'static [&'static str],
}

const VERSION_FLAG: &[&str] = &["--version"];

impl Runtime {
    /// The runtime's name as its users know it (a language or runtime name, never translated).
    pub const fn label(self) -> &'static str {
        match self {
            Self::Node => "Node",
            Self::Deno => "Deno",
            Self::Python => "Python",
            Self::Lua => "Lua",
            Self::Go => "Go",
            Self::Ruby => "Ruby",
            Self::Php => "PHP",
        }
    }

    /// The version probe.
    pub const fn probe(self) -> Probe {
        match self {
            Self::Node => Probe { executables: &["node"], args: VERSION_FLAG },
            Self::Deno => Probe { executables: &["deno"], args: VERSION_FLAG },
            Self::Python => Probe { executables: &["python3", "python"], args: VERSION_FLAG },
            Self::Lua => Probe { executables: &["lua", "lua5.4", "luajit"], args: &["-v"] },
            Self::Go => Probe { executables: &["go"], args: &["version"] },
            Self::Ruby => Probe { executables: &["ruby"], args: VERSION_FLAG },
            Self::Php => Probe { executables: &["php"], args: VERSION_FLAG },
        }
    }
}

/// The version a probe printed: the first run of digits and dots that holds a dot, read from
/// the first line that has one (`v20.11.1`, `Python 3.12.2`, `go version go1.22.0 …`).
pub fn parse_version(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        line.split(|c: char| !(c.is_ascii_digit() || c == '.'))
            .map(|run| run.trim_matches('.'))
            .find(|run| run.contains('.') && run.starts_with(|c: char| c.is_ascii_digit()))
            .map(str::to_string)
    })
}

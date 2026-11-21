//! Scripting and query languages: Python, JavaScript, TypeScript, shell and SQL.

use super::super::grammar::{Grammar, BOTH_QUOTES, C_BLOCK, C_LINE, HASH_LINE, PLAIN};

pub(super) const PYTHON: Grammar = Grammar {
    names: &["python", "py", "python3"],
    line_comments: HASH_LINE,
    quotes: BOTH_QUOTES,
    keywords: &[
        "and", "as", "assert", "async", "await", "break", "case", "class", "continue", "def", "del", "elif", "else", "except",
        "False", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "match", "None", "nonlocal", "not", "or",
        "pass", "raise", "return", "self", "True", "try", "while", "with", "yield",
    ],
    types: &["bool", "bytes", "dict", "float", "int", "list", "object", "set", "str", "tuple"],
    capitalized_types: true,
    ..PLAIN
};

const JS_KEYWORDS: &[&str] = &[
    "async", "await", "break", "case", "catch", "class", "const", "continue", "debugger", "default", "delete", "do", "else",
    "export", "extends", "false", "finally", "for", "from", "function", "if", "import", "in", "instanceof", "let", "new", "null",
    "of", "return", "static", "super", "switch", "this", "throw", "true", "try", "typeof", "undefined", "var", "void", "while",
    "yield",
];

pub(super) const JAVASCRIPT: Grammar = Grammar {
    names: &["javascript", "js", "jsx", "mjs", "cjs"],
    line_comments: C_LINE,
    block_comment: C_BLOCK,
    quotes: &['"', '\'', '`'],
    multiline_quotes: &['`'],
    keywords: JS_KEYWORDS,
    capitalized_types: true,
    ..PLAIN
};

pub(super) const TYPESCRIPT: Grammar = Grammar {
    names: &["typescript", "ts", "tsx"],
    keywords: &[
        "abstract", "as", "async", "await", "break", "case", "catch", "class", "const", "continue", "declare", "default", "delete",
        "do", "else", "enum", "export", "extends", "false", "finally", "for", "from", "function", "if", "implements", "import", "in",
        "instanceof", "interface", "keyof", "let", "namespace", "new", "null", "of", "private", "protected", "public", "readonly",
        "return", "static", "super", "switch", "this", "throw", "true", "try", "type", "typeof", "undefined", "var", "void", "while",
        "yield",
    ],
    types: &["any", "bigint", "boolean", "never", "number", "object", "string", "symbol", "unknown"],
    ..JAVASCRIPT
};

pub(super) const SHELL: Grammar = Grammar {
    names: &["shell", "sh", "bash", "zsh", "console", "shellscript"],
    line_comments: HASH_LINE,
    quotes: BOTH_QUOTES,
    keywords: &[
        "case", "do", "done", "echo", "elif", "else", "esac", "exit", "export", "fi", "for", "function", "if", "in", "local",
        "return", "set", "source", "then", "unset", "until", "while",
    ],
    ..PLAIN
};

pub(super) const SQL: Grammar = Grammar {
    names: &["sql", "sqlite", "postgres", "postgresql", "mysql"],
    line_comments: &["--"],
    block_comment: C_BLOCK,
    quotes: &['\''],
    keywords: &[
        "add", "all", "alter", "and", "as", "asc", "between", "by", "case", "create", "default", "delete", "desc", "distinct", "drop",
        "else", "end", "exists", "false", "foreign", "from", "group", "having", "in", "index", "inner", "insert", "into", "is",
        "join", "key", "left", "like", "limit", "not", "null", "offset", "on", "or", "order", "outer", "primary", "references",
        "returning", "right", "select", "set", "table", "then", "true", "union", "update", "values", "when", "where", "with",
    ],
    types: &[
        "bigint", "blob", "boolean", "char", "date", "double", "float", "int", "integer", "numeric", "real", "serial", "text",
        "timestamp", "varchar",
    ],
    case_insensitive: true,
    ..PLAIN
};

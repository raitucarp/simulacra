//! Compiled, C-family languages: Rust, Go, C, C++, Java, C# and Kotlin.

use super::super::grammar::{Grammar, BOTH_QUOTES, C_BLOCK, C_LINE, PLAIN};

const DOUBLE_QUOTE: &[char] = &['"'];

pub(super) const RUST: Grammar = Grammar {
    names: &["rust", "rs"],
    line_comments: C_LINE,
    block_comment: C_BLOCK,
    quotes: DOUBLE_QUOTE,
    char_literals: true,
    keywords: &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if",
        "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super",
        "trait", "true", "type", "unsafe", "use", "where", "while",
    ],
    types: &[
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "bool", "char", "str",
    ],
    capitalized_types: true,
    ..PLAIN
};

pub(super) const GO: Grammar = Grammar {
    names: &["go", "golang"],
    line_comments: C_LINE,
    block_comment: C_BLOCK,
    quotes: &['"', '\'', '`'],
    multiline_quotes: &['`'],
    keywords: &[
        "break", "case", "chan", "const", "continue", "default", "defer", "else", "fallthrough", "false", "for", "func", "go", "goto",
        "if", "import", "interface", "iota", "map", "nil", "package", "range", "return", "select", "struct", "switch", "true", "type",
        "var",
    ],
    types: &[
        "any", "bool", "byte", "complex64", "complex128", "error", "float32", "float64", "int", "int8", "int16", "int32", "int64",
        "rune", "string", "uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
    ],
    ..PLAIN
};

const C_KEYWORDS: &[&str] = &[
    "auto", "break", "case", "const", "continue", "default", "do", "else", "enum", "extern", "false", "for", "goto", "if", "inline",
    "register", "return", "sizeof", "static", "struct", "switch", "true", "typedef", "union", "volatile", "while", "NULL",
];
const C_TYPES: &[&str] = &[
    "bool", "char", "double", "float", "int", "long", "short", "signed", "unsigned", "void", "size_t", "int8_t", "int16_t",
    "int32_t", "int64_t", "uint8_t", "uint16_t", "uint32_t", "uint64_t",
];

pub(super) const C: Grammar = Grammar {
    names: &["c", "h"],
    line_comments: C_LINE,
    block_comment: C_BLOCK,
    quotes: BOTH_QUOTES,
    keywords: C_KEYWORDS,
    types: C_TYPES,
    ..PLAIN
};

pub(super) const CPP: Grammar = Grammar {
    names: &["cpp", "c++", "cc", "cxx", "hpp"],
    keywords: &[
        "auto", "break", "case", "catch", "class", "const", "constexpr", "continue", "default", "delete", "do", "else", "enum",
        "explicit", "extern", "false", "for", "friend", "if", "inline", "namespace", "new", "noexcept", "nullptr", "operator",
        "override", "private", "protected", "public", "return", "sizeof", "static", "struct", "switch", "template", "this", "throw",
        "true", "try", "typedef", "typename", "union", "using", "virtual", "while",
    ],
    capitalized_types: true,
    ..C
};

pub(super) const JAVA: Grammar = Grammar {
    names: &["java"],
    keywords: &[
        "abstract", "assert", "break", "case", "catch", "class", "continue", "default", "do", "else", "enum", "extends", "false",
        "final", "finally", "for", "if", "implements", "import", "instanceof", "interface", "new", "null", "package", "private",
        "protected", "public", "record", "return", "static", "super", "switch", "synchronized", "this", "throw", "throws", "true",
        "try", "var", "void", "volatile", "while",
    ],
    types: &["boolean", "byte", "char", "double", "float", "int", "long", "short"],
    capitalized_types: true,
    ..C
};

pub(super) const CSHARP: Grammar = Grammar {
    names: &["csharp", "cs", "c#"],
    keywords: &[
        "abstract", "as", "async", "await", "base", "break", "case", "catch", "class", "const", "continue", "default", "delegate",
        "do", "else", "enum", "event", "false", "finally", "for", "foreach", "get", "if", "in", "interface", "internal", "is", "lock",
        "namespace", "new", "null", "out", "override", "private", "protected", "public", "readonly", "record", "ref", "return",
        "sealed", "set", "static", "struct", "switch", "this", "throw", "true", "try", "typeof", "using", "var", "virtual", "void",
        "while",
    ],
    types: &["bool", "byte", "char", "decimal", "double", "float", "int", "long", "object", "short", "string", "uint", "ulong"],
    capitalized_types: true,
    ..C
};

pub(super) const KOTLIN: Grammar = Grammar {
    names: &["kotlin", "kt", "kts"],
    quotes: DOUBLE_QUOTE,
    char_literals: true,
    keywords: &[
        "as", "break", "class", "continue", "data", "do", "else", "false", "for", "fun", "if", "import", "in", "interface",
        "internal", "is", "null", "object", "open", "override", "package", "private", "public", "return", "sealed", "super", "this",
        "throw", "true", "try", "typealias", "val", "var", "when", "while",
    ],
    types: &[],
    capitalized_types: true,
    ..C
};

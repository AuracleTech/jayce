use super::regexify;
use regex::Regex;

lazy_static::lazy_static! (
pub static ref DUOS_RUST: Vec<(&'static str, Regex)> = vec![
    (
        "keyword",
        regexify!(
            r"^(mut|let|if|else|fn|struct|enum|match|use|mod|pub|crate|impl|trait|for|while|loop|break|continue|return|as|const|static|type|where|unsafe|extern|ref|self|super|in|move|dyn|abstract|async|await|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield)\b"
        ),
    ),
    ("string", regexify!(r#"^"[^"]*""#)),
    ("char", regexify!(r"^'(.|\\n)'")),
    ("lifetime", regexify!(r"^'(?:[a-z_][a-z0-9_]*|static)\b")),
    ("operator", regexify!(r"^(=|\+|-|\*|/|%)")),
    ("identifier", regexify!(r"^[a-zA-Z_][a-zA-Z0-9_]*")),
    ("integer", regexify!(r"^\d+")),
    ("float", regexify!(r"^\d+\.\d+")),
    ("double_colon", regexify!(r"^::")),
    ("semicolon", regexify!(r"^;")),
    ("open_brace", regexify!(r"^\{")),
    ("close_brace", regexify!(r"^\}")),
    ("open_paren", regexify!(r"^\(")),
    ("close_paren", regexify!(r"^\)")),
    ("open_bracket", regexify!(r"^\[")),
    ("close_bracket", regexify!(r"^\]")),
    ("comma", regexify!(r"^,")),
    ("hash", regexify!(r"^#")),
    ("dot", regexify!(r"^\.")),
    ("colon", regexify!(r"^:")),
    ("pipe", regexify!(r"^\|")),
    ("open_angle", regexify!(r"^<")),
    ("close_angle", regexify!(r"^>")),
    ("caret", regexify!(r"^\^")),
    ("temp_borrow", regexify!(r"^&")),
    ("question", regexify!(r"^\?")),
    ("macro_exclamation", regexify!(r"^!")),
];
);

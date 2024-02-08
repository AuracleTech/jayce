use regex::Regex;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Duos {
    Whitespace,
    CommentLine,
    CommentBlock,
    Newline,

    Keyword,
    String,
    Char,
    Lifetime,
    Operator,
    Identifier,
    Integer,
    Float,
    DoubleColon,
    Semicolon,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Hash,
    Dot,
    Colon,
    Pipe,
    OpenAngle,
    CloseAngle,
    Caret,
    TempBorrow,
    Question,
    MacroExclamation,
}

lazy_static::lazy_static! {
pub static ref DUOS_RUST: Vec<(Duos, Regex)> =  crate::duos!(
Duos::Whitespace, r"^[^\S\n]+",
Duos::CommentLine, r"^//(.*)",
Duos::CommentBlock, r"^/\*(.|\n)*?\*/",
Duos::Newline, r"^\n",

Duos::Keyword, r"^(mut|let|if|else|fn|struct|enum|match|use|mod|pub|crate|impl|trait|for|while|loop|break|continue|return|as|const|static|type|where|unsafe|extern|ref|self|super|in|move|dyn|abstract|async|await|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield)\b",
Duos::String, r#"^"[^"]*""#,
Duos::Char, r"^'(.|\\n)'",
Duos::Lifetime, r"^'(?:[a-z_][a-z0-9_]*|static)\b",
Duos::Operator, r"^(=|\+|-|\*|/|%)",
Duos::Identifier, r"^[a-zA-Z_][a-zA-Z0-9_]*",
Duos::Integer, r"^\d+",
Duos::Float, r"^\d+\.\d+",
Duos::DoubleColon, r"^::",
Duos::Semicolon, r"^;",
Duos::OpenBrace, r"^\{",
Duos::CloseBrace, r"^\}",
Duos::OpenParen, r"^\(",
Duos::CloseParen, r"^\)",
Duos::OpenBracket, r"^\[",
Duos::CloseBracket, r"^\]",
Duos::Comma, r"^,",
Duos::Hash, r"^#",
Duos::Dot, r"^\.",
Duos::Colon, r"^:",
Duos::Pipe, r"^\|",
Duos::OpenAngle, r"^<",
Duos::CloseAngle, r"^>",
Duos::Caret, r"^\^",
Duos::TempBorrow, r"^&",
Duos::Question, r"^\?",
Duos::MacroExclamation, r"^!"
);
}

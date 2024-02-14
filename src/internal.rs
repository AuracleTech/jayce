use crate::Duo;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum KindsRust {
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
    pub static ref DUOS_RUST: Vec<Duo<KindsRust>> = vec![
        Duo::new(KindsRust::Whitespace, r"^[^\S\n]+", false),
        Duo::new(KindsRust::CommentLine, r"^//(.*)", false),
        Duo::new(KindsRust::CommentBlock, r"^/\*(.|\n)*?\*/", false),
        Duo::new(KindsRust::Newline, r"^\n", false),
        Duo::new(
            KindsRust::Keyword,
            r"^(mut|let|if|else|fn|struct|enum|match|use|mod|pub|crate|impl|trait|for|while|loop|break|continue|return|as|const|static|type|where|unsafe|extern|ref|self|super|in|move|dyn|abstract|async|await|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield)\b",
            true
        ),
        Duo::new(KindsRust::String, r#"^"[^"]*""#, true),
        Duo::new(KindsRust::Char, r"^'(.|\\n)'", true),
        Duo::new(KindsRust::Lifetime, r"^'(?:[a-z_][a-z0-9_]*|static)\b", true),
        Duo::new(KindsRust::Operator, r"^(=|\+|-|\*|/|%)", true),
        Duo::new(KindsRust::Identifier, r"^[a-zA-Z_][a-zA-Z0-9_]*", true),
        Duo::new(KindsRust::Integer, r"^\d+", true),
        Duo::new(KindsRust::Float, r"^\d+\.\d+", true),
        Duo::new(KindsRust::DoubleColon, r"^::", true),
        Duo::new(KindsRust::Semicolon, r"^;", true),
        Duo::new(KindsRust::OpenBrace, r"^\{", true),
        Duo::new(KindsRust::CloseBrace, r"^\}", true),
        Duo::new(KindsRust::OpenParen, r"^\(", true),
        Duo::new(KindsRust::CloseParen, r"^\)", true),
        Duo::new(KindsRust::OpenBracket, r"^\[", true),
        Duo::new(KindsRust::CloseBracket, r"^\]", true),
        Duo::new(KindsRust::Comma, r"^,", true),
        Duo::new(KindsRust::Hash, r"^#", true),
        Duo::new(KindsRust::Dot, r"^\.", true),
        Duo::new(KindsRust::Colon, r"^:", true),
        Duo::new(KindsRust::Pipe, r"^\|", true),
        Duo::new(KindsRust::OpenAngle, r"^<", true),
        Duo::new(KindsRust::CloseAngle, r"^>", true),
        Duo::new(KindsRust::Caret, r"^\^", true),
        Duo::new(KindsRust::TempBorrow, r"^&", true),
        Duo::new(KindsRust::Question, r"^\?", true),
        Duo::new(KindsRust::MacroExclamation, r"^!", true),
    ];
}

use super::regexify;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Duos {
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

lazy_static::lazy_static! (
pub static ref DUOS_RUST: Vec<(Duos, Regex)> =  vec![
        (
            Duos::Keyword,
            regexify!(
                r"^(mut|let|if|else|fn|struct|enum|match|use|mod|pub|crate|impl|trait|for|while|loop|break|continue|return|as|const|static|type|where|unsafe|extern|ref|self|super|in|move|dyn|abstract|async|await|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield)\b"
            ),
        ),
        (Duos::String, regexify!(r#"^"[^"]*""#)),
        (Duos::Char, regexify!(r"^'(.|\\n)'")),
        (
            Duos::Lifetime,
            regexify!(r"^'(?:[a-z_][a-z0-9_]*|static)\b"),
        ),
        (Duos::Operator, regexify!(r"^(=|\+|-|\*|/|%)")),
        (Duos::Identifier, regexify!(r"^[a-zA-Z_][a-zA-Z0-9_]*")),
        (Duos::Integer, regexify!(r"^\d+")),
        (Duos::Float, regexify!(r"^\d+\.\d+")),
        (Duos::DoubleColon, regexify!(r"^::")),
        (Duos::Semicolon, regexify!(r"^;")),
        (Duos::OpenBrace, regexify!(r"^\{")),
        (Duos::CloseBrace, regexify!(r"^\}")),
        (Duos::OpenParen, regexify!(r"^\(")),
        (Duos::CloseParen, regexify!(r"^\)")),
        (Duos::OpenBracket, regexify!(r"^\[")),
        (Duos::CloseBracket, regexify!(r"^\]")),
        (Duos::Comma, regexify!(r"^,")),
        (Duos::Hash, regexify!(r"^#")),
        (Duos::Dot, regexify!(r"^\.")),
        (Duos::Colon, regexify!(r"^:")),
        (Duos::Pipe, regexify!(r"^\|")),
        (Duos::OpenAngle, regexify!(r"^<")),
        (Duos::CloseAngle, regexify!(r"^>")),
        (Duos::Caret, regexify!(r"^\^")),
        (Duos::TempBorrow, regexify!(r"^&")),
        (Duos::Question, regexify!(r"^\?")),
        (Duos::MacroExclamation, regexify!(r"^!")),
    ];
);

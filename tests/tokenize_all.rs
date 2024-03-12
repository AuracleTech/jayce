use jayce::{Duo, Token, Tokenizer};
use std::sync::OnceLock;

const SOURCE: &str = "abc 123 xyz456 // comment";
const SOURCE_PANIC: &str = "🦀";

#[derive(Debug, PartialEq, Clone)]
pub enum Kinds {
    CommentLine,
    Whitespace,

    Alpha,
    Numeric,
}

fn duos() -> &'static Vec<Duo<Kinds>> {
    static DUOS: OnceLock<Vec<Duo<Kinds>>> = OnceLock::new();
    DUOS.get_or_init(|| {
        vec![
            Duo::new(Kinds::CommentLine, r"^//.*$", false),
            Duo::new(Kinds::Whitespace, r"^\s+", false),
            Duo::new(Kinds::Alpha, r"^[a-zA-Z]+", true),
            Duo::new(Kinds::Numeric, r"^\d+", true),
        ]
    })
}

const EXPECTED: [Token<Kinds>; 4] = [
    Token {
        kind: &Kinds::Alpha,
        value: "abc",
        pos: (1, 1),
    },
    Token {
        kind: &Kinds::Numeric,
        value: "123",
        pos: (1, 5),
    },
    Token {
        kind: &Kinds::Alpha,
        value: "xyz",
        pos: (1, 9),
    },
    Token {
        kind: &Kinds::Numeric,
        value: "456",
        pos: (1, 12),
    },
];

#[test]
fn tokenize_all() {
    let tokens = Tokenizer::new(SOURCE, duos()).consume_all().unwrap();
    assert_eq!(tokens, EXPECTED);
    assert_eq!(tokens.len(), EXPECTED.len());
}

#[test]
#[should_panic(expected = "Failed to match at line")]
fn tokenize_all_should_panic() {
    let mut tokenizer = Tokenizer::new(SOURCE_PANIC, duos());
    let _ = tokenizer.consume_all().unwrap();
}

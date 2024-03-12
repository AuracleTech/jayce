use jayce::{Duo, Token, Tokenizer};
use std::sync::OnceLock;

const SOURCE: &str = r#"let kind_cat = "I calmly pet my cute cats"
pancake_icecream

very_multiline

"#;

fn duos() -> &'static Vec<Duo<&'static str>> {
    static DUOS: OnceLock<Vec<Duo<&'static str>>> = OnceLock::new();
    DUOS.get_or_init(|| {
        vec![
            Duo::new("whitespace", r"^[^\S\n]+", true),
            Duo::new("comment_line", r"^//(.*)", true),
            Duo::new("comment_block", r"^/\*(.|\n)*?\*/", true),
            Duo::new("newline", r"^\n", true),
            Duo::new("operator", r"^=", true),
            Duo::new("keyword", r"^let", true),
            Duo::new("string", r#"^"[^"]*""#, true),
            Duo::new("identifier", r"^[a-z_]+", true),
        ]
    })
}

const EXPECTED: [Token<&'static str>; 14] = [
    Token {
        kind: &"keyword",
        value: "let",
        pos: (1, 1),
    },
    Token {
        kind: &"whitespace",
        value: " ",
        pos: (1, 4),
    },
    Token {
        kind: &"identifier",
        value: "kind_cat",
        pos: (1, 5),
    },
    Token {
        kind: &"whitespace",
        value: " ",
        pos: (1, 13),
    },
    Token {
        kind: &"operator",
        value: "=",
        pos: (1, 14),
    },
    Token {
        kind: &"whitespace",
        value: " ",
        pos: (1, 15),
    },
    Token {
        kind: &"string",
        value: "\"I calmly pet my cute cats\"",
        pos: (1, 16),
    },
    Token {
        kind: &"newline",
        value: "\n",
        pos: (1, 43),
    },
    Token {
        kind: &"identifier",
        value: "pancake_icecream",
        pos: (2, 1),
    },
    Token {
        kind: &"newline",
        value: "\n",
        pos: (2, 17),
    },
    Token {
        kind: &"newline",
        value: "\n",
        pos: (3, 1),
    },
    Token {
        kind: &"identifier",
        value: "very_multiline",
        pos: (4, 1),
    },
    Token {
        kind: &"newline",
        value: "\n",
        pos: (4, 15),
    },
    Token {
        kind: &"newline",
        value: "\n",
        pos: (5, 1),
    },
];

#[test]
fn peek_test() {
    let mut tokenizer = Tokenizer::new(SOURCE, duos());
    let mut tokens = Vec::new();

    if let Some(token) = tokenizer.peek().unwrap() {
        assert_eq!(token, EXPECTED[0]);
    }
    if let Some(token) = tokenizer.peek().unwrap() {
        assert_eq!(token, EXPECTED[0]);
    }

    for expected in EXPECTED.iter() {
        let peeked = tokenizer.peek().unwrap();
        let consumed = tokenizer.consume().unwrap();
        assert_eq!(peeked, consumed);
        tokens.push(consumed.unwrap());
        if consumed.is_some() {
            assert_eq!(consumed.unwrap(), *expected);
        } else {
            panic!("Early termination, expected token.")
        }
    }

    let token = tokenizer.consume().unwrap();
    assert_eq!(token, None);
    let token = tokenizer.peek().unwrap();
    assert_eq!(token, None);
    let token = tokenizer.consume().unwrap();
    assert_eq!(token, None);
    let token = tokenizer.peek().unwrap();
    assert_eq!(token, None);
    let token = tokenizer.consume().unwrap();
    assert_eq!(token, None);

    assert_eq!(tokens, EXPECTED);
    assert_eq!(tokens.len(), EXPECTED.len());
}

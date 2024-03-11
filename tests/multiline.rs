use jayce::{Duo, Token, Tokenizer};

const SOURCE: &str = r#"let kind_cat = "I calmly pet my cute cats"
pancake_icecream

very_multiline

"#;

lazy_static::lazy_static! {
    static ref DUOS: Vec<Duo<&'static str>> = vec![
        Duo::new("whitespace", r"^[^\S\n]+", true),
        Duo::new("comment_line", r"^//(.*)", true),
        Duo::new("comment_block", r"^/\*(.|\n)*?\*/", true),
        Duo::new("newline", r"^\n", true),

        Duo::new("operator", r"^=", true),
        Duo::new("keyword", r"^let", true),
        Duo::new("string", r#"^"[^"]*""#, true),
        Duo::new("identifier", r"^[a-z_]+", true)
    ];
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
fn multiline() {
    let tokens = Tokenizer::new(SOURCE, &DUOS).tokenize_all().unwrap();
    assert_eq!(tokens, EXPECTED);
    assert_eq!(tokens.len(), EXPECTED.len());
}

use jayce::{Tokenizer, TokenizerResult};

#[test]
fn multiline_example() {
    let mut jayce = Tokenizer::new();
    jayce.add("newline", r"^\n");
    jayce.add("whitespace", r"^\s+");
    jayce.add("keyword", r"^let");
    jayce.add("assign", r"^=");
    jayce.add("string", r#"^"[^"]*""#);
    jayce.add("identifier", r"^[a-z_]+");

    let source = "let dead_cat = \"I mix my cat in a blender\"\nNOTHINGelse";

    let mut truth = vec![
        ("keyword", "let", 1, 4),
        ("whitespace", " ", 1, 5),
        ("identifier", "dead_cat", 1, 13),
        ("whitespace", " ", 1, 14),
        ("assign", "=", 1, 15),
        ("whitespace", " ", 1, 16),
        ("string", "\"I mix my cat in a blender\"", 1, 43),
        ("newline", "\n", 2, 1),
    ];

    for expected in truth.drain(..) {
        let (kind, value, line, column) = expected;

        match jayce.next(source) {
            TokenizerResult::Token(token) => {
                assert_eq!(kind, jayce.kinds[token.kind]);
                assert_eq!(value, &source[token.start..token.end]);
                assert_eq!(token.line, line);
                assert_eq!(token.column, column);
            }
            _ => panic!("Expected token, got something else"),
        }
    }
}

#[test]
fn readme_example() {
    let mut jayce = Tokenizer::new();
    jayce.add("newline", r"^\n");
    jayce.add("whitespace", r"^\s+");
    jayce.add("name", r"^[a-zA-Z_]+");
    jayce.add("price", r"^[0-9]+\$");
    jayce.add("equals", r"^=");

    let source = "Excalibur = 5000$";

    let mut truth = vec![
        ("name", "Excalibur", 1, 10),
        ("whitespace", " ", 1, 11),
        ("equals", "=", 1, 12),
        ("whitespace", " ", 1, 13),
        ("price", "5000$", 1, 18),
    ];

    for expected in truth.drain(..) {
        let (kind, value, line, column) = expected;
        match jayce.next(source) {
            TokenizerResult::Token(token) => {
                assert_eq!(kind, jayce.kinds[token.kind]);
                assert_eq!(value, &source[token.start..token.end]);
                assert_eq!(token.line, line);
                assert_eq!(token.column, column);
            }
            _ => panic!("No token found when expected"),
        }
    }
}

#[test]
#[should_panic]
fn bad_format_regex() {
    let mut jayce = Tokenizer::new();
    jayce.add("newline", "(? WRONG REGEX");
}

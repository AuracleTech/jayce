use jayce::{Tokenizer, TokenizerResult};

#[test]
fn multiline_example() {
    let source = "let dead_cat = \"I mix my cat in a blender\"\nNOTHINGelse";
    let duos = [
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("keyword", r"^let"),
        ("assign", r"^="),
        ("string", r#"^"[^"]*""#),
        ("identifier", r"^[a-z_]+"),
    ];
    let mut jayce = Tokenizer::new(source, &duos);

    let mut truth = vec![
        ("keyword", "let", (1, 4)),
        ("whitespace", " ", (1, 5)),
        ("identifier", "dead_cat", (1, 13)),
        ("whitespace", " ", (1, 14)),
        ("assign", "=", (1, 15)),
        ("whitespace", " ", (1, 16)),
        ("string", "\"I mix my cat in a blender\"", (1, 43)),
        ("newline", "\n", (2, 1)),
    ];

    for expected in truth.drain(..) {
        let (kind, value, pos) = expected;
        match jayce.next() {
            TokenizerResult::Found(token) => {
                assert_eq!(token.kind, kind);
                assert_eq!(token.value, value);
                assert_eq!(token.pos, pos);
            }
            _ => panic!("Expected token, got something else"),
        }
    }
}

#[test]
fn readme_example() {
    let source = "Excalibur = 5000$";
    let duos = vec![
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("name", r"^[a-zA-Z_]+"),
        ("price", r"^[0-9]+\$"),
        ("equals", r"^="),
    ];
    let mut jayce = Tokenizer::new(source, &duos);

    let mut truth = vec![
        ("name", "Excalibur", (1, 10)),
        ("whitespace", " ", (1, 11)),
        ("equals", "=", (1, 12)),
        ("whitespace", " ", (1, 13)),
        ("price", "5000$", (1, 18)),
    ];

    for expected in truth.drain(..) {
        let (kind, value, pos) = expected;
        match jayce.next() {
            TokenizerResult::Found(token) => {
                assert_eq!(token.kind, kind);
                assert_eq!(token.value, value);
                assert_eq!(token.pos, pos);
            }
            _ => panic!("No token found when expected"),
        }
    }
}

#[test]
#[should_panic]
fn bad_format_regex() {
    let source = "Excalibur = 5000$";
    let duos = vec![("newline", "(? WRONG REGEX")];
    let _jayce = Tokenizer::new(source, &duos);
}

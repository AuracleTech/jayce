use jayce::Tokenizer;

#[test]
fn verify_tokenizer() {
    let source = "let dead_cat = \"I mix my cat in a blender\"\nWA";
    let duos = &[
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("keyword", r"^let"),
        ("assign", r"^="),
        ("string", r#"^"[^"]*""#),
        ("identifier", r"^[a-z][a-z_]+"),
    ];
    let mut expected = vec![
        (Some("keyword"), "let", 1, 4),
        (Some("whitespace"), " ", 1, 5),
        (Some("identifier"), "dead_cat", 1, 13),
        (Some("whitespace"), " ", 1, 14),
        (Some("assign"), "=", 1, 15),
        (Some("whitespace"), " ", 1, 16),
        (Some("string"), "\"I mix my cat in a blender\"", 1, 43),
        (Some("newline"), "\n", 2, 1),
        (None, "W", 2, 2),
        (None, "A", 2, 3),
    ];
    let mut jayce = Tokenizer::new(source, duos);

    while let Some(token) = jayce.eat() {
        let (kind, value, line, column) = expected.remove(0);
        assert_eq!(token.kind, kind);
        assert_eq!(token.value, value);
        assert_eq!(token.line, line);
        assert_eq!(token.column, column);
    }
}

#[test]

fn meme() {
    let source = "Excalibur = 5000$";
    let duos = &[
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("name", r"^[a-zA-Z_]+"),
        ("price", r"^[0-9]+\$"),
        ("equals", r"^="),
    ];
    let mut jayce = Tokenizer::new(source, duos);

    while let Some(token) = jayce.eat() {
        println!("{:?}", token);
    }
}

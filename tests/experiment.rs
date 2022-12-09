use jayce::Jayce;

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
    let expected = vec![
        ("keyword", "let", 1, 4),
        ("whitespace", " ", 1, 5),
        ("identifier", "dead_cat", 1, 13),
        ("whitespace", " ", 1, 14),
        ("assign", "=", 1, 15),
        ("whitespace", " ", 1, 16),
        ("string", "\"I mix my cat in a blender\"", 1, 43),
        ("newline", "\n", 2, 1),
        ("unknown", "W", 2, 2),
        ("unknown", "A", 2, 3),
    ];
    let eat_count_expected = expected.len();
    let mut jayce = Jayce::new(source, duos);
    assert_eq!(jayce.source, source);
    while let Some(token) = jayce.eat() {
        let (kind, value, line, column) = expected[jayce.eat_count - 1];
        assert_eq!(token.kind, kind);
        assert_eq!(token.value, value);
        assert_eq!(token.line, line);
        assert_eq!(token.column, column);
    }
    assert_eq!(jayce.eat_count, eat_count_expected);
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
    let mut jayce = Jayce::new(source, duos);

    while let Some(token) = jayce.eat() {
        println!("{:?}", token);
    }
}

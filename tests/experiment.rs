use jayce::Jayce;

#[test]
fn verify_tokenizer_can_eat_every_token() {
    let source = "let dead_cat = \"I put my cat in a blender\"\nWA";
    let duos = &[
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("let", r"^let"),
        ("assign", r"^="),
        ("string", r#"^"[^"]*""#),
        ("identifier", r"^[a-z][a-z_]+"),
    ];
    let verif = vec![
        ("let", "let", 1, 4),
        ("whitespace", " ", 1, 5),
        ("identifier", "dead_cat", 1, 13),
        ("whitespace", " ", 1, 14),
        ("assign", "=", 1, 15),
        ("whitespace", " ", 1, 16),
        ("string", "\"I put my cat in a blender\"", 1, 43),
        ("newline", "\n", 2, 1),
        ("unknown", "W", 2, 2),
        ("unknown", "A", 2, 3),
        ("eof", "", 2, 3),
    ];
    let mut jayce = Jayce::new(source, duos);
    while let Some(token) = jayce.eat() {
        let (kind, value, line, column) = verif[jayce.eat_count - 1];
        assert_eq!(token.kind, kind);
        assert_eq!(token.value, value);
        assert_eq!(token.line, line);
        assert_eq!(token.column, column);
    }
}

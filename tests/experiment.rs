use jayce::Jayce;

#[test]
fn verify_kind_and_value() {
    let duos = &[
        ("WhiteSpace", r"^\s+"),
        ("Let", r"^let"),
        ("Assign", r"^="),
        ("String", r#"^"[^"]*""#),
        ("Identifier", r"^[a-z][a-z_]+"),
    ];
    let source = "let dead_cat = \"I put my cat in a blender\"";
    let mut jayce = Jayce::new(source, duos);
    let verif = vec![
        ("Let", "let"),
        ("WhiteSpace", " "),
        ("Identifier", "dead_cat"),
        ("WhiteSpace", " "),
        ("Assign", "="),
        ("WhiteSpace", " "),
        ("String", "\"I put my cat in a blender\""),
        ("EoF", "End of File"),
    ];
    for (kind, value) in verif {
        let token = jayce.eat();
        assert_eq!(kind, token.kind);
        assert_eq!(value, token.value);
    }
}

#[test]
fn verify_line_and_column() {
    let duos = &[
        ("Text", r#"^[a-zA-Z][a-zA-Z_?!]+"#),
        ("Whitespace", r"^\s+"),
    ];
    let source = "WHAT!\nYour_cat_DIED?\n\n\n\nImpressive";
    let mut jayce = Jayce::new(source, duos);
    let verif = vec![
        (1, 6),
        (2, 1),
        (2, 15),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 1),
        (6, 11),
    ];
    for (line, column) in verif {
        let token = jayce.eat();
        assert_eq!(line, token.line);
        assert_eq!(column, token.column);
    }
}

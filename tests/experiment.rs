use jayce::Jayce;

#[test]
fn verify_kind_and_value() {
    let duos: &[(&str, &str)] = &[
        ("WhiteSpace", r"^\s+"),
        ("Let", r"^let"),
        ("Assign", r"^="),
        ("String", r#"^"[^"]*""#),
        ("Identifier", r"^[a-z][a-z_]+"),
    ];
    let source = "let dead_cat = \"I put my cat in a blender\"";
    //&str means
    let mut jayce = Jayce::new(source, duos);
    let token = jayce.eat();
    assert_eq!(token.value, "let");
    assert_eq!(token.kind, "Let");
    let token = jayce.eat();
    assert_eq!(token.value, " ");
    assert_eq!(token.kind, "WhiteSpace");
    let token = jayce.eat();
    assert_eq!(token.value, "dead_cat");
    assert_eq!(token.kind, "Identifier");
    let token = jayce.eat();
    assert_eq!(token.value, " ",);
    assert_eq!(token.kind, "WhiteSpace");
    let token = jayce.eat();
    assert_eq!(token.value, "=");
    assert_eq!(token.kind, "Assign");
    let token = jayce.eat();
    assert_eq!(token.value, " ",);
    assert_eq!(token.kind, "WhiteSpace");
    let token = jayce.eat();
    assert_eq!(token.value, "\"I put my cat in a blender\"");
    assert_eq!(token.kind, "String");
    let token = jayce.eat();
    assert_eq!(token.value, "End of File");
    assert_eq!(token.kind, "EoF");
}

#[test]
fn verify_line_and_column() {
    let duos: &[(&str, &str)] = &[
        ("Text", r#"^[a-zA-Z][a-zA-Z_?!]+"#),
        ("Whitespace", r"^\s+"),
    ];
    let source = "WHAT!\nYour_cat_DIED?\n\n\n\nImpressive";
    let mut jayce = Jayce::new(source, duos);
    let token = jayce.eat();
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 6);
    let token = jayce.eat();
    assert_eq!(token.line, 2);
    assert_eq!(token.column, 15);
    let token = jayce.eat();
    assert_eq!(token.line, 6);
    assert_eq!(token.column, 11);
    let token = jayce.eat();
    assert_eq!(token.line, 6);
    assert_eq!(token.column, 11);
}

use crate::Jayce;

#[test]
fn token_recognition() {
    let duos: Vec<(&str, &str)> = vec![
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
fn line_and_column() {
    let duos: Vec<(&str, &str)> = vec![("Text", r#"^[a-zA-Z][a-zA-Z_]+"#), ("Whitespace", r"^\s+")];
    let source = "WHAT\nYOUR_CAT_DIED\n\n\n\nImpressive";
    let mut jayce = Jayce::new(source, duos);
    let token = jayce.eat();
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 5);
    let token = jayce.eat();
    assert_eq!(token.line, 2);
    assert_eq!(token.column, 14);
    let token = jayce.eat();
    assert_eq!(token.line, 6);
    assert_eq!(token.column, 11);
    let token = jayce.eat();
    assert_eq!(token.line, 6);
    assert_eq!(token.column, 11);
}

#[test]
fn readme_example() {
    let duos: Vec<(&str, &str)> = vec![
        ("WhiteSpace", r"^\s+"),
        ("identifiers", "^[a-z][a-z_]*"),
        ("number", "^[0-9]+"),
        ("operator", "^[-+*/%]"),
    ];

    let source = "exam_result = 89/100";

    let mut jayce = Jayce::new(source, duos);

    println!("{:?}", jayce.eat());
}

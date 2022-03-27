use crate::{sonant::Sonant, Jayce};

#[test]
fn token_recognition() {
    let sonants: Vec<Sonant> = vec![
        Sonant::new("WhiteSpace", r"^\s+"),
        Sonant::new("Let", r"^let"),
        Sonant::new("Equals", r"^="),
        Sonant::new("String", r#"^"[^"]*""#),
        Sonant::new("Number", r"^[0-9]+"),
        Sonant::new("Identifier", r"^[a-z][a-z_]+"),
    ];
    let source = String::from("let dead_cat = \"I put my cat in a blender\"");
    let mut jayce = Jayce::new(source, sonants);
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
    assert_eq!(token.kind, "Equals");
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
    let sonants: Vec<Sonant> = vec![
        Sonant::new("Text", r#"^[a-zA-Z][a-zA-Z_]+"#),
        Sonant::new("Whitespace", r"^\s+"),
    ];
    let source = String::from("WHAT\nYOUR_CAT_DIED\n\n\n\nImpressive");
    let mut jayce = Jayce::new(source, sonants);
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

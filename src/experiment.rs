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
    let mut tokenizer = Jayce::new(source, sonants);
    assert_eq!(tokenizer.peek().value, "let");
    assert_eq!(tokenizer.eat().kind, "Let");
    assert_eq!(tokenizer.peek().value, " ");
    assert_eq!(tokenizer.eat().kind, "WhiteSpace");
    assert_eq!(tokenizer.peek().value, "dead_cat");
    assert_eq!(tokenizer.eat().kind, "Identifier");
    assert_eq!(tokenizer.peek().value, " ",);
    assert_eq!(tokenizer.eat().kind, "WhiteSpace");
    assert_eq!(tokenizer.peek().value, "=");
    assert_eq!(tokenizer.eat().kind, "Equals");
    assert_eq!(tokenizer.peek().value, " ",);
    assert_eq!(tokenizer.eat().kind, "WhiteSpace");
    assert_eq!(tokenizer.peek().value, "\"I put my cat in a blender\"");
    assert_eq!(tokenizer.eat().kind, "String");
    assert_eq!(tokenizer.peek().value, "End of File");
    assert_eq!(tokenizer.eat().kind, "EoF");
}

#[test]
fn line_and_column() {
    let sonants: Vec<Sonant> = vec![
        Sonant::new("Text", r#"^[a-zA-Z][a-zA-Z_]+"#),
        Sonant::new("Whitespace", r"^\s+"),
    ];
    let source = String::from("WHAT\nYOUR_CAT_DIED\n\n\n\nImpressive");
    let mut tokenizer = Jayce::new(source, sonants);
    assert_eq!(tokenizer.peek().line, 1);
    assert_eq!(tokenizer.eat().column, 5);
    assert_eq!(tokenizer.peek().line, 2);
    assert_eq!(tokenizer.eat().column, 14);
    assert_eq!(tokenizer.peek().line, 6);
    assert_eq!(tokenizer.eat().column, 11);
    assert_eq!(tokenizer.peek().line, 6);
    assert_eq!(tokenizer.eat().column, 11);
}

use jayce::{
    internal::{KindsRust, DUOS_RUST},
    Duo, Tokenizer,
};
use lazy_static::lazy_static;

// Custom duos

#[derive(Debug, PartialEq, Clone)]
enum CustomKinds {
    Whitespace,
    CommentLine,
    CommentBlock,
    Newline,

    Name,
    Operator,
    Price,
}

const SOURCE_CUSTOMKINDS: &str = "Excalibur = 5000$";

lazy_static! {
    static ref DUOS_CUSTOMKINDS: Vec<Duo<CustomKinds>> = vec![
        Duo::new(CustomKinds::Whitespace, r"^\s+", false),
        Duo::new(CustomKinds::CommentLine, r"^//(.*)", false),
        Duo::new(CustomKinds::CommentBlock, r"^/\*(.|\n)*?\*/", false),
        Duo::new(CustomKinds::Newline, r"^\n", false),
        Duo::new(CustomKinds::Price, r"^[0-9]+\$", true),
        Duo::new(CustomKinds::Operator, r"^=", true),
        Duo::new(CustomKinds::Name, r"^[a-zA-Z_]+", true)
    ];
}

const EXPECTED_CUSTOMDUOS: &[(CustomKinds, &str, (usize, usize))] = &[
    (CustomKinds::Name, "Excalibur", (1, 1)),
    (CustomKinds::Whitespace, " ", (1, 10)),
    (CustomKinds::Operator, "=", (1, 11)),
    (CustomKinds::Whitespace, " ", (1, 12)),
    (CustomKinds::Price, "5000$", (1, 13)),
];

#[test]
fn custom_duos() {
    verify(SOURCE_CUSTOMKINDS, &DUOS_CUSTOMKINDS, EXPECTED_CUSTOMDUOS);
}

// Single line

const SOURCE_SINGLELINE: &str = "Excalibur = 5000$";

lazy_static::lazy_static! {
    static ref DUOS_SINGLELINE: Vec<Duo<&'static str>> = vec![
        Duo::new("whitespace", r"^\s+", false),
        Duo::new("comment_line", r"^//(.*)", false),
        Duo::new("comment_block", r"^/\*(.|\n)*?\*/", false),
        Duo::new("newline", r"^\n", false),

        Duo::new("price", r"^[0-9]+\$", true),
        Duo::new("operator", r"^=", true),
        Duo::new("name", r"^[a-zA-Z_]+", true)
    ];
}

const EXPECTED_SINGLELINE: &[(&str, &str, (usize, usize))] = &[
    ("name", "Excalibur", (1, 1)),
    ("whitespace", " ", (1, 10)),
    ("operator", "=", (1, 11)),
    ("whitespace", " ", (1, 12)),
    ("price", "5000$", (1, 13)),
];

#[test]
fn singleline() {
    verify(SOURCE_SINGLELINE, &DUOS_SINGLELINE, EXPECTED_SINGLELINE);
}

// Multi line

const SOURCE_MULTILINE: &str = r#"let dead_cat = "I mix my cat in a blender"
pancake_icecream = yes

very_multiline

"#;

lazy_static::lazy_static! {
    static ref DUOS_MULTILINE: Vec<Duo<&'static str>> = vec![
        Duo::new("whitespace", r"^[^\S\n]+", false),
        Duo::new("comment_line", r"^//(.*)", false),
        Duo::new("comment_block", r"^/\*(.|\n)*?\*/", false),
        Duo::new("newline", r"^\n", false),

        Duo::new("operator", r"^=", true),
        Duo::new("keyword", r"^let", true),
        Duo::new("string", r#"^"[^"]*""#, true),
        Duo::new("identifier", r"^[a-z_]+", true)
    ];
}

const EXPECTED_MULTILINE: &[(&str, &str, (usize, usize))] = &[
    ("keyword", "let", (1, 1)),
    ("whitespace", " ", (1, 4)),
    ("identifier", "dead_cat", (1, 5)),
    ("whitespace", " ", (1, 13)),
    ("operator", "=", (1, 14)),
    ("whitespace", " ", (1, 15)),
    ("string", "\"I mix my cat in a blender\"", (1, 16)),
    ("newline", "\n", (1, 43)),
    ("identifier", "pancake_icecream", (2, 1)),
    ("whitespace", " ", (2, 17)),
    ("operator", "=", (2, 18)),
    ("whitespace", " ", (2, 19)),
    ("identifier", "yes", (2, 20)),
    ("newline", "\n", (2, 23)),
    ("newline", "\n", (3, 1)),
    ("identifier", "very_multiline", (4, 1)),
    ("newline", "\n", (4, 15)),
    ("newline", "\n", (5, 1)),
];

#[test]
fn multiline() {
    verify(SOURCE_MULTILINE, &DUOS_MULTILINE, EXPECTED_MULTILINE);
}

// Comments, whistepaces, newlines and block comments

const SOURCE_COMMENTS: &str = r#"// This is a comment
let a = 5; // This is another comment
let b;
/* This is a multiline \r\ncomment
that spans multiple lines */
let d = 8; // And This should be ignored `\r\n`\nlol
let f = d + e;
// this is a comment

/**///yes

/* */
/**/

// \n
"#;
const EXPECTED_COMMENTS: &[(KindsRust, &str, (usize, usize))] = &[
    (KindsRust::CommentLine, "// This is a comment", (1, 1)),
    (KindsRust::Newline, "\n", (1, 21)),
    (KindsRust::Keyword, "let", (2, 1)),
    (KindsRust::Whitespace, " ", (2, 4)),
    (KindsRust::Identifier, "a", (2, 5)),
    (KindsRust::Whitespace, " ", (2, 6)),
    (KindsRust::Operator, "=", (2, 7)),
    (KindsRust::Whitespace, " ", (2, 8)),
    (KindsRust::Integer, "5", (2, 9)),
    (KindsRust::Semicolon, ";", (2, 10)),
    (KindsRust::Whitespace, " ", (2, 11)),
    (
        KindsRust::CommentLine,
        "// This is another comment",
        (2, 12),
    ),
    (KindsRust::Newline, "\n", (2, 38)),
    (KindsRust::Keyword, "let", (3, 1)),
    (KindsRust::Whitespace, " ", (3, 4)),
    (KindsRust::Identifier, "b", (3, 5)),
    (KindsRust::Semicolon, ";", (3, 6)),
    (KindsRust::Newline, "\n", (3, 7)),
    (
        KindsRust::CommentBlock,
        "/* This is a multiline \\r\\ncomment\nthat spans multiple lines */",
        (4, 1),
    ),
    (KindsRust::Newline, "\n", (5, 29)),
    (KindsRust::Keyword, "let", (6, 1)),
    (KindsRust::Whitespace, " ", (6, 4)),
    (KindsRust::Identifier, "d", (6, 5)),
    (KindsRust::Whitespace, " ", (6, 6)),
    (KindsRust::Operator, "=", (6, 7)),
    (KindsRust::Whitespace, " ", (6, 8)),
    (KindsRust::Integer, "8", (6, 9)),
    (KindsRust::Semicolon, ";", (6, 10)),
    (KindsRust::Whitespace, " ", (6, 11)),
    (
        KindsRust::CommentLine,
        "// And This should be ignored `\\r\\n`\\nlol",
        (6, 12),
    ),
    (KindsRust::Newline, "\n", (6, 53)),
    (KindsRust::Keyword, "let", (7, 1)),
    (KindsRust::Whitespace, " ", (7, 4)),
    (KindsRust::Identifier, "f", (7, 5)),
    (KindsRust::Whitespace, " ", (7, 6)),
    (KindsRust::Operator, "=", (7, 7)),
    (KindsRust::Whitespace, " ", (7, 8)),
    (KindsRust::Identifier, "d", (7, 9)),
    (KindsRust::Whitespace, " ", (7, 10)),
    (KindsRust::Operator, "+", (7, 11)),
    (KindsRust::Whitespace, " ", (7, 12)),
    (KindsRust::Identifier, "e", (7, 13)),
    (KindsRust::Semicolon, ";", (7, 14)),
    (KindsRust::Newline, "\n", (7, 15)),
    (KindsRust::CommentLine, "// this is a comment", (8, 1)),
    (KindsRust::Newline, "\n", (8, 21)),
    (KindsRust::Newline, "\n", (9, 1)),
    (KindsRust::CommentBlock, "/**/", (10, 1)),
    (KindsRust::CommentLine, "//yes", (10, 5)),
    (KindsRust::Newline, "\n", (10, 10)),
    (KindsRust::Newline, "\n", (11, 1)),
    (KindsRust::CommentBlock, "/* */", (12, 1)),
    (KindsRust::Newline, "\n", (12, 6)),
    (KindsRust::CommentBlock, "/**/", (13, 1)),
    (KindsRust::Newline, "\n", (13, 5)),
    (KindsRust::Newline, "\n", (14, 1)),
    (KindsRust::CommentLine, "// \\n", (15, 1)),
    (KindsRust::Newline, "\n", (15, 6)),
];

#[test]
fn comments() {
    verify(SOURCE_COMMENTS, &DUOS_RUST, EXPECTED_COMMENTS);
}

// No token match

const SOURCE_NO_MATCH: &str = "~";

#[test]
#[should_panic(expected = "Failed to match")]
fn no_match() {
    verify(SOURCE_NO_MATCH, &DUOS_RUST, &[]);
}

// Unexpected token

const SOURCE_UNEXPECTED: &str = "let";

#[test]
#[should_panic(expected = "Unexpected token")]
fn unexpected() {
    verify(SOURCE_UNEXPECTED, &DUOS_RUST, &[]);
}

// Verify function

#[cfg(not(feature = "serialization"))]
fn verify<T>(source: &str, duos: &Vec<Duo<T>>, expected: &[(T, &str, (usize, usize))])
where
    T: PartialEq + std::fmt::Debug,
{
    let mut tokenizer = Tokenizer::new(source, duos);

    for (kind, value, (line, column)) in expected {
        let token = match tokenizer.next() {
            Ok(Some(token)) => token,
            Ok(None) => panic!("No token found when expected"),
            Err(err) => panic!("Error while tokenizing: {}", err),
        };

        println!(
            "Expected {:?} got {:?}",
            (kind, value, (line, column)),
            token,
        );

        assert_eq!(kind, token.kind);
        assert_eq!(value, &token.value);
        assert_eq!(line, &token.pos.0);
        assert_eq!(column, &token.pos.1);
    }

    match tokenizer.next() {
        Ok(Some(token)) => panic!("Unexpected token: {:?}", token),
        Ok(None) => {}
        Err(err) => panic!("Error while tokenizing: {}", err),
    };
}

#[cfg(feature = "serialization")]
fn verify<T>(source: &str, duos: &'static [(T, Regex)], expected: &[(T, &str, (usize, usize))])
where
    T: PartialEq + std::fmt::Debug + Clone,
{
    let mut tokenizer = Tokenizer::new(source, duos);

    for (kind, value, (line, column)) in expected {
        let token = match tokenizer.next() {
            Ok(Some(token)) => token,
            Ok(None) => panic!("No token found when expected"),
            Err(err) => panic!("Error while tokenizing: {}", err),
        };

        println!(
            "Expected {:?} got {:?}",
            (kind, value, (line, column)),
            token,
        );

        assert_eq!(kind, &token.kind);
        assert_eq!(value, &token.value);
        assert_eq!(line, &token.pos.0);
        assert_eq!(column, &token.pos.1);
    }

    match tokenizer.next() {
        Ok(Some(token)) => panic!("Unexpected token: {:?}", token),
        Ok(None) => {}
        Err(err) => panic!("Error while tokenizing: {}", err),
    };
}

#[cfg(feature = "serialization")]
#[test]
fn serialization_one() {
    use jayce::Token;
    let token = Token {
        kind: "example_kind",
        value: "example_value".to_string(),
        pos: (1, 2),
    };

    let serialized = serde_json::to_string(&token).unwrap();
    let deserialized: Token<&str> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(token, deserialized);
}

#[cfg(feature = "serialization")]
const SOURCE_SERIALIZATION: &str = r"use crate::{
    app::AppData,
    camera::Camera,
    model::{self, Model},
    texture::{self, Texture},
};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use vulkanalia::prelude::v1_0::*;";

#[cfg(feature = "serialization")]
#[test]
fn serialization_collection() {
    let mut tokenizer = Tokenizer::new(SOURCE_SERIALIZATION, &DUOS_RUST);

    let tokens = tokenizer.tokenize_all().unwrap();

    let serialized = serde_json::to_string(&tokens).unwrap();
    let mut deserialized: Vec<jayce::Token<&str>> = serde_json::from_str(&serialized).unwrap();

    while let Some(token) = tokenizer.next().unwrap() {
        let deser_token = deserialized.pop().unwrap();
        let token_kind = match deser_token.kind {
            "CommentLine" => KindsRust::CommentLine,
            "Newline" => KindsRust::Newline,
            "Keyword" => KindsRust::Keyword,
            "Whitespace" => KindsRust::Whitespace,
            "Operator" => KindsRust::Operator,
            "Identifier" => KindsRust::Identifier,
            "Integer" => KindsRust::Integer,
            "Semicolon" => KindsRust::Semicolon,
            "CommentBlock" => KindsRust::CommentBlock,
            _ => panic!("Unexpected token kind"),
        };
        assert_eq!(token.value, deser_token.value);
        assert_eq!(token.pos.0, deser_token.pos.0);
        assert_eq!(token.pos.1, deser_token.pos.1);
        assert_eq!(token.kind, token_kind);
    }
}

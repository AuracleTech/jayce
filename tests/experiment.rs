use jayce::{
    internal::{Duos, DUOS_RUST},
    regexify, Tokenizer,
};
use lazy_static::lazy_static;
use regex::Regex;

// Single line

const SOURCE_SINGLELINE: &str = "Excalibur = 5000$";

lazy_static! {
    static ref DUOS_SINGLELINE: Vec<(&'static str, Regex)> = vec![
        ("price", regexify!(r"^[0-9]+\$")),
        ("operator", regexify!(r"^=")),
        ("name", regexify!(r"^[a-zA-Z_]+")),
    ];
}

const EXPECTED_SINGLELINE: &[(&str, &str, (usize, usize))] = &[
    ("name", "Excalibur", (1, 1)),
    ("operator", "=", (1, 11)),
    ("price", "5000$", (1, 13)),
];
#[test]
fn singleline() {
    verify(SOURCE_SINGLELINE, &DUOS_SINGLELINE, EXPECTED_SINGLELINE);
}

// Multi line

const SOURCE_MULTILINE: &str = r#"let dead_cat = "I mix my cat in a blender"
pancake_icecream = yes

very_multiline"#;

lazy_static! {
    static ref DUOS_MULTILINE: Vec<(&'static str, Regex)> = vec![
        ("operator", regexify!(r"^=")),
        ("keyword", regexify!(r"^let")),
        ("string", regexify!(r#"^"[^"]*""#)),
        ("identifier", regexify!(r"^[a-z_]+")),
    ];
}

const EXPECTED_MULTILINE: &[(&str, &str, (usize, usize))] = &[
    ("keyword", "let", (1, 1)),
    ("identifier", "dead_cat", (1, 5)),
    ("operator", "=", (1, 14)),
    ("string", "\"I mix my cat in a blender\"", (1, 16)),
    ("identifier", "pancake_icecream", (2, 1)),
    ("operator", "=", (2, 18)),
    ("identifier", "yes", (2, 20)),
    ("identifier", "very_multiline", (4, 1)),
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
let d = 5; // And This should be ignored `\r\n`\nlol
let f = d + e;
// this is a comment

/**///yes

/* */
/**/

"#;
const EXPECTED_COMMENTS: &[(Duos, &str, (usize, usize))] = &[
    (Duos::Keyword, "let", (2, 1)),
    (Duos::Identifier, "a", (2, 5)),
    (Duos::Operator, "=", (2, 7)),
    (Duos::Integer, "5", (2, 9)),
    (Duos::Semicolon, ";", (2, 10)),
    (Duos::Keyword, "let", (3, 1)),
    (Duos::Identifier, "b", (3, 5)),
    (Duos::Semicolon, ";", (3, 6)),
    (Duos::Keyword, "let", (6, 1)),
    (Duos::Identifier, "d", (6, 5)),
    (Duos::Operator, "=", (6, 7)),
    (Duos::Integer, "5", (6, 9)),
    (Duos::Semicolon, ";", (6, 10)),
    (Duos::Keyword, "let", (7, 1)),
    (Duos::Identifier, "f", (7, 5)),
    (Duos::Operator, "=", (7, 7)),
    (Duos::Identifier, "d", (7, 9)),
    (Duos::Operator, "+", (7, 11)),
    (Duos::Identifier, "e", (7, 13)),
    (Duos::Semicolon, ";", (7, 14)),
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

fn verify<T>(source: &str, duos: &'static [(T, Regex)], expected: &[(T, &str, (usize, usize))])
where
    T: PartialEq + std::fmt::Debug,
{
    let mut tokenizer = Tokenizer::new(source, duos.into());

    for (kind, value, (line, column)) in expected {
        let next = match tokenizer.next() {
            Ok(Some(token)) => token,
            Ok(None) => panic!("No token found when expected"),
            Err(err) => panic!("Error while tokenizing: {}", err),
        };
        assert_eq!(kind, next.kind);
        assert_eq!(value, &next.value);
        assert_eq!(line, &next.pos.0);
        assert_eq!(column, &next.pos.1);
    }

    match tokenizer.next() {
        Ok(Some(token)) => panic!("Unexpected token: {:?}", token),
        Ok(None) => {}
        Err(err) => panic!("Error while tokenizing: {}", err),
    };
}

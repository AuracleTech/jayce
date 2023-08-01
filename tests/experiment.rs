use jayce::{internal::DUOS_RUST, regexify, Tokenizer, TokenizerResult};
use lazy_static::lazy_static;
use regex::Regex;

/*
 * Single-line
 */

lazy_static! {
    static ref DUOS_SINGLELINE: Vec<(&'static str, Regex)> = vec![
        ("price", regexify!(r"^[0-9]+\$")),
        ("operator", regexify!(r"^=")),
        ("name", regexify!(r"^[a-zA-Z_]+")),
    ];
}
const SOURCE_SINGLELINE: &str = "Excalibur = 5000$";
const EXPECTED_SINGLELINE: &[(&str, &str, (usize, usize))] = &[
    ("name", "Excalibur", (1, 1)),
    ("operator", "=", (1, 11)),
    ("price", "5000$", (1, 13)),
];
#[test]
fn singleline() {
    verify(SOURCE_SINGLELINE, &DUOS_SINGLELINE, EXPECTED_SINGLELINE);
}

/*
 * Multi-line
 */

lazy_static! {
    static ref DUOS_MULTILINE: Vec<(&'static str, Regex)> = vec![
        ("operator", regexify!(r"^=")),
        ("keyword", regexify!(r"^let")),
        ("string", regexify!(r#"^"[^"]*""#)),
        ("identifier", regexify!(r"^[a-z_]+")),
    ];
}
const SOURCE_MULTILINE: &str = r#"let dead_cat = "I mix my cat in a blender"
pancake_icecream = yes

very_multiline"#;
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

/*
 * Comments, whistepaces, newlines and block comments
 */

const SOURCE_COMMENTS: &str = r#"// This is a comment
let a = 5; // This is another comment
let b;
/* This is a multiline \r\ncomment
that spans multiple lines */
let d = 5; // And This should be ignored `\r\n`\nlol
let f = d + e;
// this is a comment"#;
const EXPECTED_COMMENTS: &[(&str, &str, (usize, usize))] = &[
    ("keyword", "let", (2, 1)),
    ("identifier", "a", (2, 5)),
    ("operator", "=", (2, 7)),
    ("integer", "5", (2, 9)),
    ("semicolon", ";", (2, 10)),
    ("keyword", "let", (3, 1)),
    ("identifier", "b", (3, 5)),
    ("semicolon", ";", (3, 6)),
    ("keyword", "let", (6, 1)),
    ("identifier", "d", (6, 5)),
    ("operator", "=", (6, 7)),
    ("integer", "5", (6, 9)),
    ("semicolon", ";", (6, 10)),
    ("keyword", "let", (7, 1)),
    ("identifier", "f", (7, 5)),
    ("operator", "=", (7, 7)),
    ("identifier", "d", (7, 9)),
    ("operator", "+", (7, 11)),
    ("identifier", "e", (7, 13)),
    ("semicolon", ";", (7, 14)),
];

#[test]
fn comments() {
    verify(SOURCE_COMMENTS, &DUOS_RUST, EXPECTED_COMMENTS);
}

fn verify(
    source: &str,
    duos: &'static [(&'static str, regex::Regex)],
    expected: &[(&str, &str, (usize, usize))],
) {
    let mut jayce = Tokenizer::new(source, duos.into());
    let mut peeked = jayce.peek();
    for (kind, value, (line, column)) in expected {
        let next = jayce.next();
        assert_eq!(peeked, next);
        match next {
            TokenizerResult::Found(token) => {
                assert_eq!(kind, &token.kind);
                assert_eq!(value, &token.value);
                assert_eq!(line, &token.pos.0);
                assert_eq!(column, &token.pos.1);
            }
            _ => panic!("No token found when expected"),
        }
        peeked = jayce.peek();
    }

    match jayce.next() {
        TokenizerResult::End => {
            assert_eq!(peeked, TokenizerResult::End);
        }
        TokenizerResult::Found(token) => {
            panic!("Unexpected token found: {:?}", token)
        }
        TokenizerResult::Error(line, column) => {
            panic!("Error line {} column {}", line, column);
        }
    }
}

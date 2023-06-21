use jayce::{internal::DUOS_RUST, regexify, Token, Tokenizer, TokenizerResult};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DUOS_SINGLELINE: Vec<(&'static str, Regex)> = vec![
        ("price", regexify!(r"^[0-9]+\$")),
        ("operator", regexify!(r"^=")),
        ("name", regexify!(r"^[a-zA-Z_]+")),
    ];
    static ref DUOS_MULTILINE: Vec<(&'static str, Regex)> = vec![
        ("operator", regexify!(r"^=")),
        ("keyword", regexify!(r"^let")),
        ("string", regexify!(r#"^"[^"]*""#)),
        ("identifier", regexify!(r"^[a-z_]+")),
    ];
}

#[test]
fn singleline() {
    let source = "Excalibur = 5000$";

    const EXPECTED: &[(&str, &str, (usize, usize))] = &[
        ("name", "Excalibur", (1, 1)),
        ("operator", "=", (1, 11)),
        ("price", "5000$", (1, 13)),
    ];

    verify(source, &DUOS_SINGLELINE, EXPECTED);
}

#[test]
fn multiline() {
    let source = "let dead_cat = \"I mix my cat in a blender\"\nNOTHINGelse";

    const EXPECTED: &[(&str, &str, (usize, usize))] = &[
        ("keyword", "let", (1, 1)),
        ("identifier", "dead_cat", (1, 5)),
        ("operator", "=", (1, 14)),
        ("string", "\"I mix my cat in a blender\"", (1, 16)),
    ];

    verify(source, &DUOS_MULTILINE, EXPECTED);
}

#[test]
fn vulkan_triangle() {
    let source = include_str!("../data/vulkan_triangle.rs");
    let mut jayce = Tokenizer::new(source, &DUOS_RUST);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        match jayce.next() {
            TokenizerResult::Found(token) => tokens.push(token),
            TokenizerResult::End => break,
            TokenizerResult::Error(line, column) => {
                panic!("Error at line {}, column {}.", line, column)
            }
        }
    }
    println!("Token count: {}", tokens.len());
}

#[test]
fn comments() {
    let source = r#"// This is a comment
let a = 5; // This is another comment
let b;
/* This is a multiline \r\ncomment
that spans multiple lines */
let d = 5; // And This should be ignored `\r\n`\nlol
let f = d + e;"#;

    const EXPECTED: &[(&str, &str, (usize, usize))] = &[
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

    verify(source, &DUOS_RUST, EXPECTED);
}

fn verify(
    source: &str,
    duos: &'static [(&'static str, regex::Regex)],
    expected: &[(&str, &str, (usize, usize))],
) {
    let mut jayce = Tokenizer::new(source, duos.into());

    for (kind, value, (line, column)) in expected {
        match jayce.next() {
            TokenizerResult::Found(token) => {
                println!("Found token: {:?}", token);
                assert_eq!(kind, &token.kind);
                assert_eq!(value, &token.value);
                assert_eq!(line, &token.pos.0);
                assert_eq!(column, &token.pos.1);
                println!("OK");
            }
            _ => panic!("No token found when expected"),
        }
    }
}

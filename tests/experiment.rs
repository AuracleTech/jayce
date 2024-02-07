use jayce::{
    duos,
    internal::{Duos, DUOS_RUST},
    Tokenizer,
};
use lazy_static::lazy_static;
use regex::Regex;

// Custom duos

#[derive(Debug, PartialEq)]
enum CustomDuos {
    Whitespace,
    CommentLine,
    CommentBlock,
    Newline,

    Name,
    Operator,
    Price,
}

const SOURCE_CUSTOMDUOS: &str = "Excalibur = 5000$";

lazy_static! {
    static ref DUOS_CUSTOMDUOS: Vec<(CustomDuos, Regex)> = duos![
        CustomDuos::Whitespace, r"^\s+",
        CustomDuos::CommentLine, r"^//(.*)",
        CustomDuos::CommentBlock, r"^/\*(.|\n)*?\*/",
        CustomDuos::Newline, r"^\n",

        CustomDuos::Price, r"^[0-9]+\$",  //
        CustomDuos::Operator, r"^=",      //
        CustomDuos::Name, r"^[a-zA-Z_]+"  //
    ];
}

const EXPECTED_CUSTOMDUOS: &[(CustomDuos, &str, (usize, usize))] = &[
    (CustomDuos::Name, "Excalibur", (1, 1)),
    (CustomDuos::Whitespace, " ", (1, 10)),
    (CustomDuos::Operator, "=", (1, 11)),
    (CustomDuos::Whitespace, " ", (1, 12)),
    (CustomDuos::Price, "5000$", (1, 13)),
];

#[test]
fn custom_duos() {
    verify(SOURCE_CUSTOMDUOS, &DUOS_CUSTOMDUOS, EXPECTED_CUSTOMDUOS);
}

// Single line

const SOURCE_SINGLELINE: &str = "Excalibur = 5000$";

lazy_static! {
    static ref DUOS_SINGLELINE: Vec<(&'static str, Regex)> = duos![
        "whitespace", r"^\s+",
        "comment_line", r"^//(.*)",
        "comment_block", r"^/\*(.|\n)*?\*/",
        "newline", r"^\n",

        "price", r"^[0-9]+\$",  //
        "operator", r"^=",      //
        "name", r"^[a-zA-Z_]+"  //
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

lazy_static! {
    static ref DUOS_MULTILINE: Vec<(&'static str, Regex)> = duos![
        "whitespace", r"^[^\S\n]+",
        "comment_line", r"^//(.*)",
        "comment_block", r"^/\*(.|\n)*?\*/",
        "newline", r"^\n",

        "operator", r"^=",         //
        "keyword", r"^let",        //
        "string", r#"^"[^"]*""#,   //
        "identifier", r"^[a-z_]+"  //
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
const EXPECTED_COMMENTS: &[(Duos, &str, (usize, usize))] = &[
    (Duos::CommentLine, "// This is a comment", (1, 1)),
    (Duos::Newline, "\n", (1, 21)),
    (Duos::Keyword, "let", (2, 1)),
    (Duos::Whitespace, " ", (2, 4)),
    (Duos::Identifier, "a", (2, 5)),
    (Duos::Whitespace, " ", (2, 6)),
    (Duos::Operator, "=", (2, 7)),
    (Duos::Whitespace, " ", (2, 8)),
    (Duos::Integer, "5", (2, 9)),
    (Duos::Semicolon, ";", (2, 10)),
    (Duos::Whitespace, " ", (2, 11)),
    (Duos::CommentLine, "// This is another comment", (2, 12)),
    (Duos::Newline, "\n", (2, 38)),
    (Duos::Keyword, "let", (3, 1)),
    (Duos::Whitespace, " ", (3, 4)),
    (Duos::Identifier, "b", (3, 5)),
    (Duos::Semicolon, ";", (3, 6)),
    (Duos::Newline, "\n", (3, 7)),
    (
        Duos::CommentBlock,
        "/* This is a multiline \\r\\ncomment\nthat spans multiple lines */",
        (4, 1),
    ),
    (Duos::Newline, "\n", (5, 29)),
    (Duos::Keyword, "let", (6, 1)),
    (Duos::Whitespace, " ", (6, 4)),
    (Duos::Identifier, "d", (6, 5)),
    (Duos::Whitespace, " ", (6, 6)),
    (Duos::Operator, "=", (6, 7)),
    (Duos::Whitespace, " ", (6, 8)),
    (Duos::Integer, "8", (6, 9)),
    (Duos::Semicolon, ";", (6, 10)),
    (Duos::Whitespace, " ", (6, 11)),
    (
        Duos::CommentLine,
        "// And This should be ignored `\\r\\n`\\nlol",
        (6, 12),
    ),
    (Duos::Newline, "\n", (6, 53)),
    (Duos::Keyword, "let", (7, 1)),
    (Duos::Whitespace, " ", (7, 4)),
    (Duos::Identifier, "f", (7, 5)),
    (Duos::Whitespace, " ", (7, 6)),
    (Duos::Operator, "=", (7, 7)),
    (Duos::Whitespace, " ", (7, 8)),
    (Duos::Identifier, "d", (7, 9)),
    (Duos::Whitespace, " ", (7, 10)),
    (Duos::Operator, "+", (7, 11)),
    (Duos::Whitespace, " ", (7, 12)),
    (Duos::Identifier, "e", (7, 13)),
    (Duos::Semicolon, ";", (7, 14)),
    (Duos::Newline, "\n", (7, 15)),
    (Duos::CommentLine, "// this is a comment", (8, 1)),
    (Duos::Newline, "\n", (8, 21)),
    (Duos::Newline, "\n", (9, 1)),
    (Duos::CommentBlock, "/**/", (10, 1)),
    (Duos::CommentLine, "//yes", (10, 5)),
    (Duos::Newline, "\n", (10, 10)),
    (Duos::Newline, "\n", (11, 1)),
    (Duos::CommentBlock, "/* */", (12, 1)),
    (Duos::Newline, "\n", (12, 6)),
    (Duos::CommentBlock, "/**/", (13, 1)),
    (Duos::Newline, "\n", (13, 5)),
    (Duos::Newline, "\n", (14, 1)),
    (Duos::CommentLine, "// \\n", (15, 1)),
    (Duos::Newline, "\n", (15, 6)),
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

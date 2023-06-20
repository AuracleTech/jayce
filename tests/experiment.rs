use jayce::{Token, Tokenizer, TokenizerResult};

const RUST_DUOS: [(&str, &str); 28] = [
    (
        "keyword",
        r"^(let|if|else|fn|struct|enum|match|use|mod|pub|crate|impl|trait|for|while|loop|break|continue|return|as|const|static|type|where|unsafe|extern|ref|self|super|in|move|dyn|abstract|async|await|become|box|do|final|macro|override|priv|typeof|unsized|virtual|yield)",
    ),
    ("string", r#"^"[^"]*""#),
    ("char", r"^'(.|\\n)'"),
    ("lifetime", r"^'(?:[a-z_][a-z0-9_]*|static)"),
    ("operator", r"^(=|\+|-|\*|/|%)"),
    ("identifier", r"^[a-zA-Z_][a-zA-Z0-9_]*"),
    ("integer", r"^\d+"),
    ("float", r"^\d+\.\d+"),
    ("double_colon", r"^::"),
    ("semicolon", r"^;"),
    ("open_brace", r"^\{"),
    ("close_brace", r"^\}"),
    ("open_paren", r"^\("),
    ("close_paren", r"^\)"),
    ("open_bracket", r"^\["),
    ("close_bracket", r"^\]"),
    ("comma", r"^,"),
    ("hash", r"^#"),
    ("dot", r"^\."),
    ("colon", r"^:"),
    ("pipe", r"^\|"),
    ("open_angle", r"^<"),
    ("close_angle", r"^>"),
    ("caret", r"^\^"),
    ("temp_borrow", r"^&"),
    ("temp_mut_borrow", r"^&mut"),
    ("question", r"^\?"),
    ("macro_exclamation", r"^!"),
];

#[test]
fn singleline() {
    let source = "Excalibur = 5000$";
    let duos = vec![
        ("price", r"^[0-9]+\$"),
        ("operator", r"^="),
        ("name", r"^[a-zA-Z_]+"),
    ];

    const EXPECTED: &[(&str, &str, (usize, usize))] = &[
        ("name", "Excalibur", (1, 1)),
        ("operator", "=", (1, 11)),
        ("price", "5000$", (1, 13)),
    ];

    verify(source, &duos, EXPECTED);
}

#[test]
fn multiline() {
    let source = "let dead_cat = \"I mix my cat in a blender\"\nNOTHINGelse";
    let duos = [
        ("operator", r"^="),
        ("keyword", r"^let"),
        ("string", r#"^"[^"]*""#),
        ("identifier", r"^[a-z_]+"),
    ];

    const EXPECTED: &[(&str, &str, (usize, usize))] = &[
        ("keyword", "let", (1, 1)),
        ("identifier", "dead_cat", (1, 5)),
        ("operator", "=", (1, 14)),
        ("string", "\"I mix my cat in a blender\"", (1, 16)),
    ];

    verify(source, &duos, EXPECTED);
}

#[test]
fn vulkan_triangle() {
    let source = include_str!("../data/vulkan_triangle.rs");
    let mut jayce = Tokenizer::new(source, RUST_DUOS.into());
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

    verify(source, &RUST_DUOS, EXPECTED);
}

fn verify(source: &str, duos: &[(&str, &str)], expected: &[(&str, &str, (usize, usize))]) {
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

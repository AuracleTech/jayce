use jayce::{Duo, Token, Tokenizer};
use lazy_static::lazy_static;

const SOURCE: &str = "Excalibur = 5000$";

lazy_static! {
    static ref DUOS: Vec<Duo<u64>> = vec![
        Duo::new(1, r"^\s+", false),
        Duo::new(2, r"^//(.*)", false),
        Duo::new(3, r"^/\*(.|\n)*?\*/", false),
        Duo::new(4, r"^\n", false),
        Duo::new(5, r"^[0-9]+\$", true),
        Duo::new(6, r"^=", true),
        Duo::new(7, r"^[a-zA-Z_]+", true)
    ];
}

const EXPECTED: [Token<u64>; 3] = [
    Token {
        kind: &7,
        value: "Excalibur",
        pos: (1, 1),
    },
    Token {
        kind: &6,
        value: "=",
        pos: (1, 11),
    },
    Token {
        kind: &5,
        value: "5000$",
        pos: (1, 13),
    },
];

#[test]
fn custom_duos() {
    let tokens = Tokenizer::new(SOURCE, &DUOS).consume_all().unwrap();
    assert_eq!(tokens, EXPECTED);
    assert_eq!(tokens.len(), EXPECTED.len());
}

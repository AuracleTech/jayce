use jayce::{Duo, Token, Tokenizer};
use std::sync::OnceLock;

const SOURCE: &str = "Excalibur = 5000$";

fn duos() -> &'static Vec<Duo<u64>> {
    static DUOS: OnceLock<Vec<Duo<u64>>> = OnceLock::new();
    DUOS.get_or_init(|| {
        vec![
            Duo::new(1, r"^\s+", false),
            Duo::new(2, r"^//(.*)", false),
            Duo::new(3, r"^/\*(.|\n)*?\*/", false),
            Duo::new(4, r"^\n", false),
            Duo::new(5, r"^[0-9]+\$", true),
            Duo::new(6, r"^=", true),
            Duo::new(7, r"^[a-zA-Z_]+", true),
        ]
    })
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
    let tokens = Tokenizer::new(SOURCE, duos()).consume_all().unwrap();
    assert_eq!(tokens, EXPECTED);
    assert_eq!(tokens.len(), EXPECTED.len());
}

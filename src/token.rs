#[derive(Debug, Clone)]
pub struct Token {
    pub kind: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn from(kind: &str, value: &str, line: usize, column: usize) -> Token {
        Self {
            kind: kind.to_owned(),
            value: value.to_owned(),
            line,
            column,
        }
    }
}

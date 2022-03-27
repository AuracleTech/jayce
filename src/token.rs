#[derive(Debug)]
pub struct Token<'a> {
    pub kind: &'a str,
    pub value: &'a str,
    pub line: u32,
    pub column: u32,
}

impl<'a> Token<'a> {
    pub fn from(kind: &'a str, value: &'a str, line: u32, column: u32) -> Token<'a> {
        Self {
            kind,
            value,
            line,
            column,
        }
    }
}

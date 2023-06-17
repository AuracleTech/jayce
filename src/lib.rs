use regex::Regex;

pub struct Tokenizer {
    pub kinds: Vec<String>,
    regexes: Vec<Regex>,
    cursor: usize,
    line: u32,
    column: u32,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: usize,
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: u32,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            kinds: vec![],
            regexes: vec![],
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn add(&mut self, kind: &str, regex: &str) {
        self.kinds.push(kind.to_string());
        self.regexes.push(Regex::new(regex).expect("Invalid regex"));
    }

    pub fn eat(&mut self, source: &str) -> Option<Token> {
        if self.cursor >= source.len() {
            return None;
        }

        for (index, regex) in self.regexes.iter().enumerate() {
            if let Some(matched) = regex.find(&source[self.cursor..]) {
                let value = matched.as_str();

                self.cursor += matched.end();
                let new_line_count = value.matches('\n').count() as u32;
                self.line += new_line_count;
                self.column = if new_line_count > 0 {
                    value.len() as u32
                } else {
                    self.column + value.len() as u32
                };

                return Some(Token {
                    kind: index,
                    start: self.cursor - matched.end(),
                    end: self.cursor,
                    line: self.line,
                    column: self.column,
                });
            }
        }

        panic!("No token at line {} col {}", self.line, self.column);
    }
}

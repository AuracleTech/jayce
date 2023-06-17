pub struct Tokenizer {
    pub kinds: Vec<String>,
    regexes: Vec<regex::Regex>,
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

pub enum TokenizerResult {
    Token(Token),
    Nothing(String),
    End,
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
        self.regexes
            .push(regex::Regex::new(regex).expect("Invalid regex"));
    }

    pub fn next(&mut self, source: &str) -> TokenizerResult {
        if self.cursor >= source.len() {
            return TokenizerResult::End;
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

                return TokenizerResult::Token(Token {
                    kind: index,
                    start: self.cursor - matched.end(),
                    end: self.cursor,
                    line: self.line,
                    column: self.column,
                });
            }
        }

        TokenizerResult::Nothing(format!("No token line {} col {}", self.line, self.column))
    }
}

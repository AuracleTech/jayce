use regex::Regex;

pub struct Tokenizer<'a> {
    source: &'a str,
    duos: Vec<(&'a str, Regex)>,
    cursor: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: &'a str,
    pub value: &'a str,
    pub pos: (usize, usize),
}

pub enum TokenizerResult<'a> {
    Found(Token<'a>),
    Unknown(String),
    End,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str, duos: &[(&'a str, &str)]) -> Tokenizer<'a> {
        let duos = duos
            .iter()
            .map(|&(k, v)| (k, Regex::new(v).expect("Invalid regex.")))
            .collect();
        Self {
            source,
            duos,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> TokenizerResult<'a> {
        if self.cursor >= self.source.len() {
            return TokenizerResult::End;
        }

        for (kind, regex) in &self.duos {
            if let Some(result) = regex.find(&self.source[self.cursor..]) {
                let value = result.as_str();
                let newlines = value.chars().filter(|&c| c == '\n').count();

                self.cursor += value.len();
                self.line += newlines;
                self.column = if newlines > 0 {
                    value.len()
                } else {
                    self.column + value.len()
                };

                return TokenizerResult::Found(Token {
                    kind,
                    value,
                    pos: (self.line, self.column),
                });
            }
        }

        TokenizerResult::Unknown(format!("No token line {} col {}", self.line, self.column))
    }
}

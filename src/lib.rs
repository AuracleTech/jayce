use regex::Regex;

pub struct Tokenizer<'a> {
    source: &'a str,
    duos: Vec<(&'a str, Regex)>,
    cursor: usize,
    line: u32,
    column: u32,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: Option<&'a str>,
    pub value: &'a str,
    pub line: u32,
    pub column: u32,
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

    pub fn eat(&mut self) -> Option<Token<'a>> {
        if self.cursor >= self.source.len() {
            return None;
        }

        let mut kind: Option<&'a str> = None;
        let mut value = &self.source[self.cursor..self.cursor + 1];
        let mut newlines = 0;

        for (duo_kind, duo_regex) in &self.duos {
            if let Some(result) = duo_regex.find(&self.source[self.cursor..]) {
                kind = Some(duo_kind);
                value = result.as_str();
                newlines = value.chars().filter(|&c| c == '\n').count() as u32;
                break;
            }
        }

        self.cursor += value.len();
        self.line += newlines;
        self.column = if newlines > 0 {
            value.len() as u32
        } else {
            self.column + value.len() as u32
        };

        Some(Token {
            kind,
            value,
            line: self.line,
            column: self.column,
        })
    }
}

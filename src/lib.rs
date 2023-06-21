use lazy_static::lazy_static;
use regex::Regex;

pub mod internal;

#[macro_export]
macro_rules! regexify {
    ($regex:expr) => {
        Regex::new($regex).expect("Invalid regex.")
    };
}

lazy_static! {
    static ref MERGED: Regex = regexify!(r"(^\s+)|(^/\*(.|\n)*?\*/)|(^//(.*)\n)");
}

pub struct Tokenizer<'a> {
    source: &'a str,
    duos: &'static [(&'static str, Regex)],
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
    Error(usize, usize),
    End,
}

impl<'a> Tokenizer<'a> {
    #[inline]
    pub fn new(source: &'a str, duos: &'static [(&'static str, Regex)]) -> Self {
        Self {
            source,
            duos,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.cursor >= self.source.len()
    }

    pub fn next(&mut self) -> TokenizerResult<'a> {
        if self.is_eof() {
            return TokenizerResult::End;
        }

        loop {
            if let Some(result) = MERGED.find(&self.source[self.cursor..]) {
                let len = result.len();
                self.cursor += len;

                let value: &str = result.as_str();
                let newlines_count = value.chars().filter(|&c| c == '\n').count();
                if newlines_count > 0 {
                    self.line += newlines_count;
                    let distance_newline = value.rfind('\n').unwrap_or(0);
                    self.column = len - distance_newline;
                } else {
                    self.column += len;
                }

                if self.is_eof() {
                    return TokenizerResult::End;
                }
            } else {
                break;
            }
        }

        for (kind, regex) in self.duos.iter() {
            if let Some(result) = regex.find(&self.source[self.cursor..]) {
                let len = result.len();
                self.cursor += len;

                let token = TokenizerResult::Found(Token {
                    kind,
                    value: result.as_str(),
                    pos: (self.line, self.column),
                });

                self.column += len;

                return token;
            }
        }

        TokenizerResult::Error(self.line, self.column)
    }
}

pub mod internal;
use regex::Regex;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! duos(($($kind:expr, $pattern:expr),*) => { vec![ $( ($kind, Regex::new($pattern).unwrap()) ),* ] };);

pub struct Tokenizer<'a, T> {
    source: &'a str,
    duos: &'a [(T, Regex)],
    cursor: usize,
    line: usize,
    column: usize,
}

#[cfg(not(feature = "serialization"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a, T> {
    pub kind: &'a T,
    pub value: &'a str,
    pub pos: (usize, usize),
}

#[cfg(not(feature = "serialization"))]
impl<'a, T> Tokenizer<'a, T> {
    #[inline]
    pub fn new(source: &'a str, duos: &'static [(T, Regex)]) -> Self {
        Self {
            source,
            duos,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> Result<Option<Token<'a, T>>, Box<dyn std::error::Error>> {
        if self.cursor >= self.source.len() {
            return Ok(None);
        }

        for (kind, regex) in self.duos.iter() {
            if let Some(result) = regex.find(&self.source[self.cursor..]) {
                let value: &str = result.as_str();

                let token = Token {
                    kind,
                    value,
                    pos: (self.line, self.column),
                };

                let len = result.len();
                self.cursor += len;
                let newlines_count = bytecount::count(value.as_bytes(), b'\n');
                if newlines_count > 0 {
                    self.line += newlines_count;
                    self.column = len - value.rfind('\n').unwrap_or(1);
                } else {
                    self.column += len;
                }

                return Ok(Some(token));
            }
        }

        Err(format!(
            "Failed to match at line {}, column {}.",
            self.line, self.column
        ))?
    }

    pub fn tokenize_all(&mut self) -> Result<Vec<Token<'a, T>>, Box<dyn std::error::Error>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next()? {
            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[cfg(feature = "serialization")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token<T> {
    pub kind: T,
    pub value: String,
    pub pos: (usize, usize),
}

#[cfg(feature = "serialization")]
impl<'a, T> Tokenizer<'a, T>
where
    T: Clone,
{
    #[inline]
    pub fn new(source: &'a str, duos: &'static [(T, Regex)]) -> Self {
        Self {
            source,
            duos,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> Result<Option<Token<T>>, Box<dyn std::error::Error>> {
        if self.cursor >= self.source.len() {
            return Ok(None);
        }

        for (kind, regex) in self.duos.iter() {
            if let Some(result) = regex.find(&self.source[self.cursor..]) {
                let value: &str = result.as_str();

                let token = Token {
                    kind: kind.clone(),
                    value: value.to_string(),
                    pos: (self.line, self.column),
                };

                let len = result.len();
                self.cursor += len;
                let newlines_count = bytecount::count(value.as_bytes(), b'\n');
                if newlines_count > 0 {
                    self.line += newlines_count;
                    self.column = len - value.rfind('\n').unwrap_or(1);
                } else {
                    self.column += len;
                }

                return Ok(Some(token));
            }
        }

        Err(format!(
            "Failed to match at line {}, column {}.",
            self.line, self.column
        ))?
    }

    pub fn tokenize_all(&mut self) -> Result<Vec<Token<T>>, Box<dyn std::error::Error>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next()? {
            tokens.push(token);
        }

        Ok(tokens)
    }
}

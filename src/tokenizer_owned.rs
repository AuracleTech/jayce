use crate::{Duo, Tokenizer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeekResult<T> {
    Match(Token<T>),
    Skipped,
    End,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token<T> {
    pub kind: T,
    pub value: String,
    pub pos: (usize, usize),
}

impl<'a, T> Tokenizer<'a, T>
where
    T: Clone,
{
    #[inline]
    pub fn new(source: &'a str, duos: &'a [Duo<T>]) -> Self {
        Self {
            source,
            duos,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn seek(&mut self) -> Result<SeekResult<T>, Box<dyn std::error::Error>> {
        if self.cursor >= self.source.len() {
            return Ok(SeekResult::End);
        }

        for duo in self.duos.iter() {
            if let Some(result) = duo.regex.find(&self.source[self.cursor..]) {
                let value: &str = result.as_str();

                let token = if duo.preserve {
                    SeekResult::Match(Token {
                        kind: duo.kind.clone(),
                        value: value.to_string(),
                        pos: (self.line, self.column),
                    })
                } else {
                    SeekResult::Skipped
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

                return Ok(token);
            }
        }

        Err(format!(
            "Failed to match at line {}, column {}.",
            self.line, self.column
        ))?
    }

    pub fn tokenize_all(&mut self) -> Result<Vec<Token<T>>, Box<dyn std::error::Error>> {
        let mut tokens = Vec::new();
        while let Ok(tokenize_result) = self.seek() {
            match tokenize_result {
                SeekResult::Match(token) => tokens.push(token),
                SeekResult::Skipped => continue,
                SeekResult::End => break,
            }
        }

        Ok(tokens)
    }
}

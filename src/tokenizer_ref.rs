use crate::{Duo, Tokenizer};

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a, T> {
    pub kind: &'a T,
    pub value: &'a str,
    pub pos: (usize, usize),
}

impl<'a, T> Tokenizer<'a, T> {
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

    pub fn next(&mut self) -> Result<Option<Token<'a, T>>, Box<dyn std::error::Error>> {
        if self.cursor >= self.source.len() {
            return Ok(None);
        }

        for duo in self.duos.iter() {
            if let Some(result) = duo.regex.find(&self.source[self.cursor..]) {
                // if !duo.preserve {
                //     // FIX
                //     if result.start() != 0 {
                //         continue;
                //     }
                // }

                let value: &str = result.as_str();

                let token = Token {
                    kind: &duo.kind,
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

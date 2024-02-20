pub mod internal;
use regex::Regex;

pub struct Duo<T> {
    pub kind: T,
    pub regex: Regex,
    pub preserve: bool,
}

impl<T> Duo<T> {
    pub fn new(kind: T, regex: &str, preserve: bool) -> Self {
        Self {
            kind,
            regex: Regex::new(regex).unwrap(),
            preserve,
        }
    }
}

pub struct Tokenizer<'a, T> {
    source: &'a str,
    duos: &'a [Duo<T>],
    cursor: usize,
    line: usize,
    column: usize,
}

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

    pub fn consume(&mut self) -> Result<Option<Token<'a, T>>, Box<dyn std::error::Error>> {
        while self.cursor < self.source.len() {
            let mut matched = false;

            for duo in self.duos.iter() {
                if let Some(result) = duo.regex.find(&self.source[self.cursor..]) {
                    let value: &str = result.as_str();
                    let token_pos = (self.line, self.column);
                    let len = result.len();
                    self.cursor += len;
                    let newlines_count = bytecount::count(value.as_bytes(), b'\n');
                    if newlines_count > 0 {
                        self.line += newlines_count;
                        self.column = len - value.rfind('\n').unwrap_or(1);
                    } else {
                        self.column += len;
                    }

                    if duo.preserve {
                        return Ok(Some(Token {
                            kind: &duo.kind,
                            value,
                            pos: token_pos,
                        }));
                    } else {
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                return Err(format!(
                    "Failed to match at line {}, column {}.",
                    self.line, self.column
                ))?;
            }
        }

        Ok(None)
    }

    pub fn tokenize_all(&mut self) -> Result<Vec<Token<'a, T>>, Box<dyn std::error::Error>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.consume()? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}

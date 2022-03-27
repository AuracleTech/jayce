use regex::Regex;

#[cfg(test)]
mod experiment;
mod token;
pub use token::Token;

pub struct Jayce {
    source: String,
    duos: Vec<(String, Regex)>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Jayce {
    pub fn new(source: &str, duos: Vec<(&str, &str)>) -> Jayce {
        Self {
            source: source.to_owned(),
            duos: duos
                .into_iter()
                .map(|(s1, s2)| {
                    (
                        s1.to_owned(),
                        Regex::new(&s2).expect("Failed to parse regex from string."),
                    )
                })
                .collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn eat(&mut self) -> Token {
        if self.cursor >= self.source.len() {
            return Token::from("EoF", "End of File", self.line, self.column);
        }

        while self.source[self.cursor..].starts_with('\n') {
            self.line += 1;
            self.cursor += 1;
            self.column = 1;
        }

        let buffer = &self.source[self.cursor..];

        for duo in self.duos.iter() {
            let result = &duo.1.find(buffer);
            if let Some(result) = result {
                self.cursor += result.end();
                self.column += result.end();
                return Token {
                    kind: duo.0.to_owned(),
                    value: result.as_str().to_owned(),
                    line: self.line,
                    column: self.column,
                };
            }
        }
        panic!(
            "No regex match found on line {} column {}",
            self.line, self.column
        );
    }
}

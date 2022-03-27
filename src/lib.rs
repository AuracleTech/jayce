use regex::Regex;

#[cfg(test)]
mod experiment;
mod sonant;
pub use sonant::Sonant;
mod token;
pub use token::Token;

#[derive(Debug)]
pub struct Jayce {
    source: String,
    sonants: Vec<Sonant>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Jayce {
    pub fn new(source: String, sonants: Vec<Sonant>) -> Jayce {
        Self {
            source,
            sonants,
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

        for sonant in self.sonants.iter() {
            let regex = Regex::new(&sonant.regex).expect("Failed to parse regex provided.");
            let result = regex.find(&self.source[self.cursor..]);
            if let Some(result) = result {
                self.cursor += result.end();
                self.column += result.end();
                return Token {
                    kind: sonant.name.clone(),
                    value: result.as_str().to_string(),
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

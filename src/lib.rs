use regex::Regex;

#[cfg(test)]
mod experiment;
mod sonant;
use sonant::Sonant;
mod token;
use token::Token;

#[derive(Debug)]
pub struct Jayce {
    source: String,
    sonants: Vec<Sonant>,
    cursor: usize,
    current: Token,
    next: Token,
    line: usize,
    column: usize,
}

impl Jayce {
    pub fn new(source: String, sonants: Vec<Sonant>) -> Jayce {
        let mut s = Self {
            source,
            sonants,
            cursor: 0,
            current: Token::from("SoF", "Start of File", 1, 1),
            next: Token::from("SoF", "Start of File", 1, 1),
            line: 1,
            column: 1,
        };
        s.read();
        s
    }

    pub fn peek(&mut self) -> Token {
        self.next.clone()
    }

    pub fn eat(&mut self) -> Token {
        self.read();
        self.current.clone()
    }

    fn read(&mut self) {
        self.current = self.next.clone();
        if self.cursor >= self.source.len() {
            return self.next = Token::from("EoF", "End of File", self.line, self.column);
        }

        while self.source[self.cursor..].starts_with('\n') {
            self.line += 1;
            self.cursor += 1;
            self.column = 1;
        }

        for sonant in self.sonants.iter() {
            let regex = Regex::new(&sonant.regex).expect("Failed to create regular expression");
            let result = regex.find(&self.source[self.cursor..]);
            if let Some(result) = result {
                self.cursor += result.end();
                self.column += result.end();
                return self.next = Token {
                    kind: sonant.name.clone(),
                    value: result.as_str().to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
        }
        panic!(
            "No expression found to match at line {} col {}",
            self.line, self.column
        );
    }
}

pub mod internal;
use regex::Regex;

pub struct Duo<T> {
    kind: T,
    regex: Regex,
    _preserve: bool,
}

impl<T> Duo<T> {
    pub fn new(kind: T, regex: &str, _preserve: bool) -> Self {
        let regex = Regex::new(regex).unwrap();
        Self {
            kind,
            regex,
            _preserve,
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

#[cfg(feature = "serialization")]
mod tokenizer_owned;
#[cfg(feature = "serialization")]
pub use tokenizer_owned::Token;

#[cfg(not(feature = "serialization"))]
mod tokenizer_ref;
#[cfg(not(feature = "serialization"))]
pub use tokenizer_ref::Token;

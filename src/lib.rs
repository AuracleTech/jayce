pub mod internal;

#[macro_export]
macro_rules! duos(($($kind:expr, $pattern:expr),*) => { vec![ $( ($kind, Regex::new($pattern).unwrap()) ),* ] };);

pub struct Tokenizer<'a, T> {
    source: &'a str,
    duos: &'a [(T, regex::Regex)],
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

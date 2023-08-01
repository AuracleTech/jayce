# jayce

Jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{regexify, Tokenizer, TokenizerResult};
use regex::Regex;

// Your token kind names and their regexes
lazy_static::lazy_static! {
    static ref DUOS: Vec<(&'static str, Regex)> = vec![
        ("price", regexify!(r"^[0-9]+\$")),
        ("operator", regexify!(r"^=")),
        ("name", regexify!(r"^[a-zA-Z_]+")),
    ];
}
// Source to tokenize
const SOURCE: &str = "Excalibur = 5000$";

fn main() {
    let mut jayce = Tokenizer::new(SOURCE, &DUOS);

    // Print all tokens until the end of source
    loop {
        match jayce.next() {
            TokenizerResult::Found(token) => println!("{:?}", token),
            TokenizerResult::End => break,
            TokenizerResult::Error(line, column) => {
                panic!("No match line {}, column {}.", line, column)
            }
        }
    }
}
```

##### Result

```rust,ignore
Token { kind: "name", value: "Excalibur", pos: (1, 1) }
Token { kind: "operator", value: "=", pos: (1, 11) }
Token { kind: "price", value: "5000$", pos: (1, 13) }
```

##### Info

`peek` only returns a `TokenizerResult`

`next` returns a `TokenizerResult` and advances the source cursor

`TokenizerResult` can be

1. `Found(token)` If a regex matches
2. `Error(line, column)` When nothing matches
3. `End` Reaching the source ends

##### Performances

Whitespaces, block comments and comments are skipped by default for performance reasons

Initialization in `1.87 nanoseconds`

Tokenization of [49 516 428](https://github.com/AuracleTech/yuumi) tokens in `3.69 milliseconds`

> version 6.0.4 is `420.65%` faster than version 4.0.1

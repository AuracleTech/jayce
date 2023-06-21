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

`next` returns a `TokenizerResult` which can be

1. `Found(token)` If a regex matches
2. `Error(line, column)` When nothing matches
3. `End` Reaching the source ends

##### Note

Whitespaces, block comments and comments are skipped by default

##### Performances

Initialization `3.0881 nanoseconds`

> `99.999%` faster than version 4.0.1

Tokenization of [vulkan-triangle](https://github.com/vulkano-rs/vulkano/blob/master/examples/src/bin/triangle.rs) in `2.2010 nanoseconds`

> `7.6013%` faster than version 4.0.1

# jayce

jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{regexify, Tokenizer};
use regex::Regex;

const SOURCE: &str = "Excalibur = 5000$; // Your custom lang";

lazy_static::lazy_static! (
    static ref DUOS: Vec<(&'static str, Regex)> = vec![
        ("price", regexify!(r"^[0-9]+\$")),
        ("semicolon", regexify!(r"^;")),
        ("operator", regexify!(r"^=")),
        ("name", regexify!(r"^[a-zA-Z_]+")),
    ];
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new(SOURCE, &DUOS);

    while let Some(token) = tokenizer.next()? {
        println!("{:?}", token);
    }

    Ok(())
}
```

##### Result

```rust,ignore
Token { kind: "name", value: "Excalibur", pos: (1, 1) }
Token { kind: "operator", value: "=", pos: (1, 11) }
Token { kind: "price", value: "5000$", pos: (1, 13) }
Token { kind: "semicolon", value: ";", pos: (1, 18) }
```

##### Info

functions available are `next` and `tokenize_all`

the `Result` has 3 variants

1. `Ok(Some(token))` If a match is found
2. `Ok(None)` Reaching the source ends
3. `Err(error)` When nothing matches

##### Note

whitespaces, comments and block comments are skipped for performance reasons

##### Performances

initialization in `1.83 nanoseconds`

tokenization of [19 979](https://github.com/AuracleTech/yuumi) tokens in `3.69 milliseconds`

> version 6.0.4 is `420.65%` faster than version 4.0.1

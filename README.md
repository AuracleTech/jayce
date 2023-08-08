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

`next` returns a `Result` with 3 possible values

1. `Ok(Some(token))` If a match is found
2. `Ok(None)` Reaching the source ends
3. `Err(error)` When an error occurs

`tokenize_all` returns a `Result` with 2 possible values

1. `Ok(tokens)` If all tokens are found
2. `Err(error)` When an error occurs

##### Note

whitespaces, comments and block comments are skipped for performance reasons

##### Performances

initialization in `1.83 nanoseconds`

tokenization of [19 979](https://github.com/AuracleTech/yuumi) tokens in `3.59 milliseconds`

> `6.0.4` is ~ `421%` faster than version `4.0.1`

> `8.0.0` is ~ `5%` faster than version `7.0.1`

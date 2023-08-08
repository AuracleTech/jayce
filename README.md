# jayce

jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{duos, Tokenizer};
use regex::Regex;

const SOURCE: &str = "Excalibur = 5000$; // Your custom lang";

lazy_static::lazy_static! (
    static ref DUOS: Vec<(&'static str, Regex)> = duos![
        "price", r"^[0-9]+\$",
        "semicolon", r"^;",
        "operator", r"^=",
        "name", r"^[a-zA-Z_]+"
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

`next` possible `Result`

1. `Ok(Some(token))` Match is found
2. `Ok(None)` End of source
3. `Err(error)` An error occurs

`tokenize_all` possible `Result`

1. `Ok(Vec<Tokens>)` Tokens are found
2. `Err(error)` An error occurs

##### Note

whitespaces, comments and block comments are skipped for performance reasons

##### Performances

initialization in `1.83 nanoseconds`

tokenization of [19 979](https://github.com/AuracleTech/yuumi) tokens in `3.5 milliseconds`

> `6.0.4` is ~ `421%` faster than version `4.0.1`

> `7.0.2` is ~ `5%` faster than version `7.0.1`

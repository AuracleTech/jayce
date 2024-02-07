# jayce

jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{duos, Tokenizer};
use regex::Regex;

const SOURCE: &str = "Excalibur = 5000$; // Your own language!";

lazy_static::lazy_static! (
    static ref DUOS: Vec<(&'static str, Regex)> = duos![
        "whitespace", r"^[^\S\n]+",
        "comment_line", r"^//(.*)",
        "comment_block", r"^/\*(.|\n)*?\*/",
        "newline", r"^\n",

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
Token { kind: "whitespace", value: " ", pos: (1, 10) }
Token { kind: "operator", value: "=", pos: (1, 11) }
Token { kind: "whitespace", value: " ", pos: (1, 12) }
Token { kind: "price", value: "5000$", pos: (1, 13) }
Token { kind: "semicolon", value: ";", pos: (1, 18) }
Token { kind: "whitespace", value: " ", pos: (1, 19) }
Token { kind: "comment_line", value: "// Your own language!", pos: (1, 20) }
```

##### Info

`next` possible `Result`

1. `Ok(Some(token))` Match is found
2. `Ok(None)` End of source
3. `Err(error)` An error occurs

`tokenize_all` possible `Result`

1. `Ok(Vec<Tokens>)` Tokens are found
2. `Err(error)` An error occurs

##### Performances

initialization in `1.83 nanoseconds` and tokenization of [29 639](https://github.com/AuracleTech/yuumi) tokens in `3.85 milliseconds`

SIMD acceleration enabled by default, modify `Cargo.toml` as follows to disable

```toml
jayce = { version = "X.X.X", default-features = false }
```

##### Changelog

> `7.0.2` is `442%` faster than version `4.0.1` from making everything precompiled

> `9.0.0` is `30%` slower than version `8.1.0` to support custom whitespaces & comments

> `9.0.2` is `5%` faster than version `9.0.1` by enabling SIMD acceleration by default

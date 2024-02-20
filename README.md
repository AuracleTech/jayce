# jayce

jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{Duo, Tokenizer};

const SOURCE: &str = "Excalibur = 5000$; // Your own language!";

lazy_static::lazy_static! {
    static ref DUOS: Vec<Duo<&'static str>> = vec![
        Duo::new("whitespace", r"^[^\S\n]+", false),
        Duo::new("commentLine", r"^//(.*)", false),
        Duo::new("commentBlock", r"^/\*(.|\n)*?\*/", false),
        Duo::new("newline", r"^\n", false),

        Duo::new("price", r"^[0-9]+\$", true),
        Duo::new("semicolon", r"^;", true),
        Duo::new("operator", r"^=", true),
        Duo::new("name", r"^[a-zA-Z_]+", true)
    ];
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new(SOURCE, &DUOS);

    while let Some(token) = tokenizer.consume()? {
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

`Tokenizer::consume` returns `Result Option Token`

1. `Ok Some` match found
2. `Ok None` end of source
3. `Err` an error occurs

`Tokenizer::tokenize_all` returns `Result Vec Token`

1. `Ok Vec Token` tokens matched
2. `Err` an error occurs

##### Performances

initialization in ~`3 nanoseconds`
tokenization of [Yuumi](https://github.com/AuracleTech/yuumi) in ~`4 milliseconds`

##### Features

- `generic-simd`
- `runtime-dispatch-simd` default enabled, to disable modify `Cargo.toml` as follows

```toml
jayce = { version = "X.X.X", default-features = false }
```

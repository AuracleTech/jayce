# jayce

jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{Duo, SeekResult, Tokenizer};

const SOURCE: &str = "Excalibur = 5000$; // Your own language!";

lazy_static::lazy_static! {
    static ref DUOS: Vec<Duo<&'static str>> = vec![
        // Token name, regular expression, and if we preserve the token
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

    while let Ok(tokenize_result) = tokenizer.seek() {
        match tokenize_result {
            SeekResult::Match(token) => println!("{:?}", token),
            SeekResult::Skipped => continue,
            SeekResult::End => break,
        }
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

`tokenizer.seek()` returns `Result`

1. `Ok(SeekResult)` Seeking next token is successful
2. `Err(error)` An error occurs

`SeekResult`

3. `Match(Token<T>)` Match found
4. `Skipped` Match found but token is not preserved
5. `End` End of source

`tokenizer.tokenize_all()` returns `Result`

1. `Ok(Vec<Tokens>)` Tokens are found
2. `Err(error)` An error occurs

##### Performances

tokenization of [Yuumi](https://github.com/AuracleTech/yuumi) project's language tokens

- `3.8 milliseconds` with referenced tokens and serialization disabled
- `5.0 milliseconds` with owned tokens and serialization available

##### Features

- `serialization`
- `generic-simd`
- `runtime-dispatch-simd` enabled by default, to disable modify `Cargo.toml` as follows

```toml
jayce = { version = "X.X.X", default-features = false }
```

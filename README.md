# jayce

Jayce is a tokenizer 🌌

##### Example

```rust
use jayce::{Tokenizer, TokenizerResult};

let source = "Excalibur = 5000$";
let duos = vec![
    ("price", r"^[0-9]+\$"),
    ("operator", r"^="),
    ("name", r"^[a-zA-Z_]+"),
];
let mut jayce = Tokenizer::new(source, duos);

loop {
    match jayce.next() {
        TokenizerResult::Found(token) => println!("{:?}", token),
        TokenizerResult::End => break,
        TokenizerResult::Error(line, column) => {
            panic!("Error line {}, column {}.", line, column)
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

`whitespaces`, `block comments` and `comments` are skipped by default

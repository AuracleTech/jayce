# jayce

Jayce is a simple tokenizer

##### Example

```rust
use jayce::{Tokenizer, TokenizerResult};

let source = "Excalibur = 5000$";
let duos = vec![
    ("newline", r"^\n"),
    ("whitespace", r"^\s+"),
    ("name", r"^[a-zA-Z_]+"),
    ("price", r"^[0-9]+\$"),
    ("equals", r"^="),
];
let mut jayce = Tokenizer::new(source, &duos);

while let TokenizerResult::Token(token) = jayce.next() {
    println!("{:?}", token);
}
```

##### Result

```rust,ignore
Token { kind: "name", value: "Excalibur", line: 1, column: 10 }
Token { kind: "whitespace", value: " ", line: 1, column: 11 }
Token { kind: "equals", value: "=", line: 1, column: 12 }
Token { kind: "whitespace", value: " ", line: 1, column: 13 }
Token { kind: "price", value: "5000$", line: 1, column: 18 }
```

##### Info

`next` returns a `TokenizerResult` which can be

1. `Found(token)` If a regex matches
2. `Unknown(message)` When there is zero match
3. `End` Reaching the source ends

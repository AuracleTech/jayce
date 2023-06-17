# jayce

Jayce is a simple tokenizer

#### Example

```rust
let mut jayce = jayce::Tokenizer::new();
jayce.add("newline", r"^\n");
jayce.add("whitespace", r"^\s+");
jayce.add("name", r"^[a-zA-Z_]+");
jayce.add("price", r"^[0-9]+\$");
jayce.add("equals", r"^=");

let source = "Excalibur = 5000$";

while let Some(token) = jayce.eat(source) {
    println!("{:?}", token);
}
```

##### Result

```rust,ignore
Token { kind: 2, start: 0, end: 9, line: 1, column: 10 }
Token { kind: 1, start: 9, end: 10, line: 1, column: 11 }
Token { kind: 4, start: 10, end: 11, line: 1, column: 12 }
Token { kind: 1, start: 11, end: 12, line: 1, column: 13 }
Token { kind: 3, start: 12, end: 17, line: 1, column: 18 }
```

##### Info

Reaching the end of source returns `None`

Unknown characters returns a `Token` with `kind` as `None` and `value` of the unknown character

# jayce

#### Token

```rust
pub struct Token {
    pub kind: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}
```

#### Tokenizer

```rust
Jayce::new(source: &str, duos: Vec<(&str, &str)>);
```

#### Example

```rust
let duos: Vec<(&str, &str)> = vec![
    ("WhiteSpace", r"^\s+"),
    ("identifiers", "^[a-z][a-z_]*"),
    ("number", "^[0-9]+"),
    ("operator", "^[-+*/%]"),
];

let source = "exam_result = 89/100";

let mut jayce = Jayce::new(source, duos);

println!("{:?}", jayce.eat());

// Token { kind: "identifiers", value: "exam_result", line: 1, column: 12 }
```

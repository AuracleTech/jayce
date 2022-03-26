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

#### Sonant

```rust
Sonant::new(kind: &str, regex: &str);
```

#### Tokenizer

```rust
Jayce::new(source: String, sonants: Vec<Sonant>);
```

#### Example

```rust
let sonants: Vec<Sonant> = vec![
    Sonant::new("WhiteSpace", r"^\s+"),
    Sonant::new("identifiers", "^[a-z][a-z_]*"),
    Sonant::new("number", "^[0-9]+"),
    Sonant::new("operator", "^[-+*/%]"),
];

let source = "math_exam = 89/100".to_owned();

let mut jayce = Jayce::new(source, sonants);

println!("{:?}", jayce.eat());

// Token { kind: "identifiers", value: "math_exam", line: 1, column: 10 }
```

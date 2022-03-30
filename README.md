# jayce

#### Token

```rust,ignore
pub struct Token<'a> {
    pub kind: &'a str,
    pub value: &'a str,
    pub line: u32,
    pub column: u32,
}
```

End of File returns `Token` kind `EoF` value `End of File`

New Lines returns `Token` kind `NewLine` value `\n`

#### Tokenizer

```rust,ignore
Jayce::new(source: &'a str, duos: &[(&'a str, &str)]);
```

#### Example

```rust
use jayce::Jayce;

let duos = &[
    ("WhiteSpace", r"^\s+"),
    ("Name", r"^[a-z][a-z_]*"),
    ("Number", r"^[0-9]+"),
    ("Operator", r"^[-+*/%]"),
];

let source = "zoe = 34";

let mut jayce = Jayce::new(source, duos);

println!("{:?}", jayce.eat());

// Token { kind: "Name", value: "zoe", line: 1, column: 4 }
```

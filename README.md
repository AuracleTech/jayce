# jayce

Jayce is a blazing fast tokenizer

#### Example

```rust
use jayce::Jayce;

fn main () {
    let source = "Excalibur = 5000$";
    let duos = &[
        ("newline", r"^\n"),
        ("whitespace", r"^\s+"),
        ("name", r"^[a-zA-Z_]+"),
        ("price", r"^[0-9]+\$"),
        ("equals", r"^="),
    ];
    let mut jayce = Jayce::new(source, duos);

    while let Some(token) = jayce.eat() {
        println!("{:?}", token);
    }
}
```

#### Result

```rust,ignore
Token { kind: "name", value: "Excalibur", line: 1, column: 10 }
Token { kind: "whitespace", value: " ", line: 1, column: 11 }
Token { kind: "equals", value: "=", line: 1, column: 12 }
Token { kind: "whitespace", value: " ", line: 1, column: 13 }
Token { kind: "price", value: "5000$", line: 1, column: 18 }
```

#### Info

Reaching the end of source returns `None`

Unknown character returns a `Token` of kind `unknown` with value of the unknown character

```rust,ignore
pub struct Jayce<'a> {
    pub source: &'a str, // source to tokenize
    pub cursor: usize, // current position in the source
    pub line: u32, // current line
    pub column: u32, // current column
    pub eat_count: usize, // total number of tokens eaten
}
```

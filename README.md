# jayce

End of file returns no token

Unknown character returns a `Token` of kind `unknown` with value of the unknown character

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
    let token = jayce.eat();
    // Token {
    //     kind: "name",
    //     value: "excalibur",
    //     line: 1,
    //     column: 10
    // }
}
```

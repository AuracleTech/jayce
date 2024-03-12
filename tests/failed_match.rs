use jayce::{internal::duos_rust, Tokenizer};

const SOURCE: &str = "🦀";

#[test]
#[should_panic(expected = "Failed to match")]
fn failed_match() {
    let mut tokenizer = Tokenizer::new(SOURCE, duos_rust());
    let _ = tokenizer.consume().unwrap();
}

use jayce::{internal::DUOS_RUST, Tokenizer};

const SOURCE: &str = "🦀";

#[test]
#[should_panic(expected = "Failed to match")]
fn failed_match() {
    let mut tokenizer = Tokenizer::new(SOURCE, &DUOS_RUST);
    let _ = tokenizer.consume().unwrap();
}

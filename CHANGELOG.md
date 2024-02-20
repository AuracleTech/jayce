# Changelog

## [9.0.1] - 2024-02-07

- updated dependencies to latest versions
- attempt to improve performance by turning the `Tokenizer` into an iterator, performance worsened, so I reverted to the current implementation

## [9.0.0] - 2023-08-16

- users can now configure whitespace characters, replacing the previous hardcoded approach

## [10.0.0] - 2024-02-14

- `Duo` struct replacing `duos` macro
- `next` method for `Tokenizer` is now `seek`
- `SeekResult` enum for `Tokenizer::seek`

## [11.0.0] - 2024-02-18

- `SeekResult` nuked
- `Tokenizer::seek` renamed to `Tokenizer::consume`
- `Tokenizer::consume` now returns `Result<Option<Token>>`
- prevent compiler from optimizing away benchmarks properly
- tests are now in separate files
- futurproofing tests by expect matching the whole `Token` struct
- `serialization` feature nuked
- fixed `Tokenizer::tokenize_all` not returning errors properly

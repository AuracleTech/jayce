# Changelog

## [9.0.1] - 2024-02-07

- updated dependencies to latest versions
- attempt to improve performance by turning the `Tokenizer` into an iterator, performance worsened, so I reverted to the current implementation

## [9.0.0] - 2023-08-16

- replace hardcoded whitespace characters to configurable

## [10.0.0] - 2024-02-14

- `Duo` struct replacing `duos` macro
- `Tokenizer::next` is now `Tokenizer::seek`
- `Tokenizer::seek` returns `SeekResult`

## [11.0.0] - 2024-02-18

- `SeekResult` nuked
- `Tokenizer::seek` renamed to `Tokenizer::consume`
- `Tokenizer::consume` now returns `Result<Option<Token>>`
- prevent compiler from optimizing away benchmarks properly
- tests are now in separate files
- futurproofing tests by expect matching the whole `Token` struct
- `serialization` feature nuked
- fixed `Tokenizer::tokenize_all` not returning errors properly

## [12.0.0] - 2024-03-11

- implemented `Tokenizer::seek`
- renamed `Tokenizer::tokenize_all` to `Tokenizer::consume_all`

## [12.1.0] - 2024-03-12

- replace `lazy_static` with `OnceLock`, removing a dependency

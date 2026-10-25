# semver-parser/src/parser.rs:159:13

- crate: https://github.com/steveklabnik/semver-parser @ b43ab0311bd3c938dee40311e0ad11cb20a32939
- kind: panic
- location: `semver-parser/src/parser.rs:159:13`
- last API call: `semver_parser::parser::Parser::version`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Cannot have more than 255 identifiers
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
semver-parser = { git = "https://github.com/steveklabnik/semver-parser", rev = "b43ab0311bd3c938dee40311e0ad11cb20a32939" }
```

`src/main.rs`:

```rust
use semver_parser::parser::Parser;

fn main() {
    let string = include_str!("string_0.txt").to_string();
    let mut parser = Parser::new(&string).unwrap();
    let _ = parser.version();
}
```

Run with `cargo run` in this directory (package `repro_semver_parser_src_parser_rs_159_13_2d6094`).

## Output

```
thread 'main' (472666) panicked at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:159:13:
Cannot have more than 255 identifiers
stack backtrace:
   0: std::panicking::begin_panic::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:751:5
   1: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:159:13
   2: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   3: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   4: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   5: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   6: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   7: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   8: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
   9: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  10: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  11: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  12: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  13: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  14: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  15: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  16: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  17: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  18: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  19: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  20: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  21: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  22: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  23: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  24: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  25: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  26: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  27: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  28: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  29: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  30: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  31: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  32: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  33: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  34: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  35: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  36: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  37: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  38: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  39: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  40: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  41: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  42: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  43: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  44: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  45: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  46: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  47: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  48: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  49: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  50: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  51: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  52: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  53: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  54: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  55: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  56: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  57: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  58: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  59: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  60: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  61: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  62: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  63: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  64: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  65: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  66: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  67: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  68: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  69: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  70: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  71: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  72: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  73: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  74: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  75: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  76: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  77: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  78: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  79: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  80: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  81: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  82: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  83: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  84: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  85: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  86: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  87: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  88: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  89: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  90: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  91: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
  92: <semver_parser::parser::Parser>::bounded_identifier
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/parser.rs:177:31
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

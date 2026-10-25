# semver-parser/src/range.rs:481:76

- crate: https://github.com/steveklabnik/semver-parser @ b43ab0311bd3c938dee40311e0ad11cb20a32939
- kind: panic
- location: `semver-parser/src/range.rs:481:76`
- last API call: `semver_parser::RangeSet::parse`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: PosOverflow }
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
semver-parser = { git = "https://github.com/steveklabnik/semver-parser", rev = "b43ab0311bd3c938dee40311e0ad11cb20a32939" }
```

`src/main.rs`:

```rust
use semver_parser::Compat;
use semver_parser::RangeSet;
use semver_parser::version::Identifier;

fn main() {
    let identifier = Identifier::Numeric(4294967295u64);
    let string = identifier.to_string();
    let identifier2 = identifier.clone().concat(&string);
    let string2 = identifier2.to_string();
    let _ = RangeSet::parse(&string2, Compat::Npm);
}
```

Run with `cargo run` in this directory (package `repro_semver_parser_src_range_rs_481_76_0dd8a8`).

## Output

```
thread 'main' (472792) panicked at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:481:76:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: PosOverflow }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<u64, core::num::error::ParseIntError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: semver_parser::range::simple::parse_partial
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:481:76
   5: semver_parser::range::simple::from_pair_iterator
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:164:39
   6: semver_parser::range::from_pair_iterator
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:128:39
   7: semver_parser::range_set::from_pair_iterator
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range_set.rs:70:27
   8: <semver_parser::range_set::RangeSet>::parse
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range_set.rs:35:9
   9: repro_semver_parser_src_range_rs_481_76_0dd8a8::main
             at ./src/main.rs:10:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

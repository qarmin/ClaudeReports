# semver-parser/src/range.rs:77:27

- crate: https://github.com/steveklabnik/semver-parser @ b43ab0311bd3c938dee40311e0ad11cb20a32939
- kind: panic
- location: `semver-parser/src/range.rs:77:27`
- last API call: `semver_parser::RangeSet::from_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
semver-parser = { git = "https://github.com/steveklabnik/semver-parser", rev = "b43ab0311bd3c938dee40311e0ad11cb20a32939" }
```

`src/main.rs`:

```rust
use semver_parser::version::Version;

fn main() {
    let version = Version { major: 0u64, minor: u64::MAX, patch: 0u64, pre: vec![], build: vec![] };
    let string = version.to_string();
    let _ = <semver_parser::RangeSet as ::core::str::FromStr>::from_str(&string);
}
```

Run with `cargo run` in this directory (package `repro_semver_parser_src_range_rs_77_27_81b087`).

## Output

```
thread 'main' (472708) panicked at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:77:27:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <semver_parser::range::Partial>::inc_minor
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:77:27
   4: semver_parser::range::simple::handle_caret_range
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:375:34
   5: semver_parser::range::simple::from_pair_iterator
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:192:37
   6: semver_parser::range::from_pair_iterator
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range.rs:128:39
   7: semver_parser::range_set::from_pair_iterator
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range_set.rs:70:27
   8: <semver_parser::range_set::RangeSet>::parse
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range_set.rs:35:9
   9: <semver_parser::range_set::RangeSet as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/git/checkouts/semver-parser-43adff33cb153e86/b43ab03/src/range_set.rs:44:9
  10: repro_semver_parser_src_range_rs_77_27_81b087::main
             at ./src/main.rs:6:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

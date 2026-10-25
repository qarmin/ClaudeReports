# gimli/src/write/str.rs:108:1

- crate: https://github.com/gimli-rs/gimli @ 64a7a194fb5fe43d474fd9c101555e5a81b1d36c
- kind: panic
- location: `gimli/src/write/str.rs:108:1`
- last API call: `gimli::write::LineString::get`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: BaseId(2)
 right: BaseId(0)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gimli = { git = "https://github.com/gimli-rs/gimli", rev = "64a7a194fb5fe43d474fd9c101555e5a81b1d36c" }
```

`src/main.rs`:

```rust
use gimli::write::LineString;
use gimli::write::LineStringTable;
use gimli::write::StringTable;

fn main() {
    let line_string = LineString::LineStringRef(LineStringTable::default().add::<Vec<u8>>(vec![]));
    let _ = line_string.get(&StringTable::default(), &LineStringTable::default());
}
```

Run with `cargo run` in this directory (package `repro_gimli_src_write_str_rs_108_1_6c3c9d`).

## Output

```
thread 'main' (430949) panicked at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/str.rs:108:1:
assertion `left == right` failed
  left: BaseId(2)
 right: BaseId(0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<gimli::write::BaseId, gimli::write::BaseId>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <gimli::write::str::LineStringTable>::get
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/str.rs:65:17
   5: <gimli::write::line::LineString>::get
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/line.rs:896:60
   6: repro_gimli_src_write_str_rs_108_1_6c3c9d::main
             at ./src/main.rs:7:25
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

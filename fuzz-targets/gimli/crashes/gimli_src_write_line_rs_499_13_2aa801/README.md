# gimli/src/write/line.rs:499:13

- crate: https://github.com/gimli-rs/gimli @ 64a7a194fb5fe43d474fd9c101555e5a81b1d36c
- kind: panic
- location: `gimli/src/write/line.rs:499:13`
- last API call: `gimli::write::LineProgram::end_sequence`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 1
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gimli = { git = "https://github.com/gimli-rs/gimli", rev = "64a7a194fb5fe43d474fd9c101555e5a81b1d36c" }
```

`src/main.rs`:

```rust
use gimli::LineEncoding;
use gimli::write::LineProgram;
use gimli::write::LineString;
use gimli::write::StringTable;

fn main() {
    let mut line_program = LineProgram::new(LineProgram::none().encoding(), LineEncoding { minimum_instruction_length: 100u8, maximum_operations_per_instruction: 1u8, default_is_stmt: false, line_base: 0i8, line_range: 1u8 }, LineString::StringRef(StringTable::default().add::<Vec<u8>>(vec![])), None, LineString::StringRef(StringTable::default().add::<Vec<u8>>(vec![])), None);
    line_program.end_sequence(1u64);
}
```

Run with `cargo run` in this directory (package `repro_gimli_src_write_line_rs_499_13_2aa801`).

## Output

```
thread 'main' (431345) panicked at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/line.rs:499:13:
assertion `left == right` failed
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<u64, u64>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <gimli::write::line::LineProgram>::op_advance
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/line.rs:499:13
   5: <gimli::write::line::LineProgram>::end_sequence
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/line.rs:365:31
   6: repro_gimli_src_write_line_rs_499_13_2aa801::main
             at ./src/main.rs:8:18
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

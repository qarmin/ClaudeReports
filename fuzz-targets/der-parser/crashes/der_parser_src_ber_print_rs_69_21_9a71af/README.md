# der-parser/src/ber/print.rs:69:21

- crate: https://github.com/rusticata/der-parser @ d14e092b7df3bea7102ee9b286e553a1bc5aef63
- kind: panic
- location: `der-parser/src/ber/print.rs:69:21`
- last API call: `der_parser::ber::PrettyBer::next_indent`
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
der-parser = { git = "https://github.com/rusticata/der-parser", rev = "d14e092b7df3bea7102ee9b286e553a1bc5aef63" }
```

`src/main.rs`:

```rust
use der_parser::ber::BerObject;

fn main() {
    let ber_object = BerObject::from_seq(vec![]);
    let pretty_ber = ber_object.as_pretty(usize::MAX, 1usize);
    let vec = vec![];
    let ber_object2 = BerObject::from_int_slice(&vec);
    let _ = pretty_ber.next_indent(&ber_object2);
}
```

Run with `cargo run` in this directory (package `repro_der_parser_src_ber_print_rs_69_21_9a71af`).

## Output

```
thread 'main' (597606) panicked at /home/rafal/.cargo/git/checkouts/der-parser-afdd110b643b94eb/d14e092/src/ber/print.rs:69:21:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <der_parser::ber::print::PrettyBer>::next_indent
             at /home/rafal/.cargo/git/checkouts/der-parser-afdd110b643b94eb/d14e092/src/ber/print.rs:69:21
   4: repro_der_parser_src_ber_print_rs_69_21_9a71af::main
             at ./src/main.rs:8:24
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

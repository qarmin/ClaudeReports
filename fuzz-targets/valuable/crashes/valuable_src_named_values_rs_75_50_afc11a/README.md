# valuable/src/named_values.rs:75:50

- crate: https://github.com/tokio-rs/valuable @ e5135dadbf1e8d7f72859889986e3bee03726f44
- kind: panic
- location: `valuable/src/named_values.rs:75:50`
- last API call: `valuable::NamedValues::get`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
valuable = { git = "https://github.com/tokio-rs/valuable", rev = "e5135dadbf1e8d7f72859889986e3bee03726f44" }
```

`src/main.rs`:

```rust
use valuable::NamedField;
use valuable::NamedValues;

fn main() {
    let string = "".to_string();
    let named_field = NamedField::new(&string);
    let vec = vec![];
    let vec2 = vec![];
    let named_values = NamedValues::new(&vec, &vec2);
    let _ = named_values.get(&named_field);
}
```

Run with `cargo run` in this directory (package `repro_valuable_src_named_values_rs_75_50_afc11a`).

## Output

```
thread 'main' (568627) panicked at /home/rafal/.cargo/git/checkouts/valuable-ff92356ff9808e69/e5135da/valuable/src/named_values.rs:75:50:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <valuable::named_values::NamedValues>::get
             at /home/rafal/.cargo/git/checkouts/valuable-ff92356ff9808e69/e5135da/valuable/src/named_values.rs:75:50
   4: repro_valuable_src_named_values_rs_75_50_afc11a::main
             at ./src/main.rs:10:26
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

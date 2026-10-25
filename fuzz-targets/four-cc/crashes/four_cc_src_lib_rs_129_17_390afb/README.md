# four-cc/src/lib.rs:129:17

- crate: https://github.com/dholroyd/four-cc @ c6a750a03f3260e3fac15643a6e0efe4fd3cae0e
- kind: panic
- location: `four-cc/src/lib.rs:129:17`
- last API call: `four_cc::FourCC::from_str`
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
four-cc = { git = "https://github.com/dholroyd/four-cc", rev = "c6a750a03f3260e3fac15643a6e0efe4fd3cae0e", features = ["schemars", "serde", "std"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <four_cc::FourCC as ::core::str::FromStr>::from_str("");
}
```

Run with `cargo run` in this directory (package `repro_four_cc_src_lib_rs_129_17_390afb`).

## Output

```
thread 'main' (520848) panicked at /home/rafal/.cargo/git/checkouts/four-cc-2fcc53021f1348b4/c6a750a/src/lib.rs:129:17:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <four_cc::FourCC as core::convert::From<&[u8]>>::from
             at /home/rafal/.cargo/git/checkouts/four-cc-2fcc53021f1348b4/c6a750a/src/lib.rs:129:17
   4: <&[u8] as core::convert::Into<four_cc::FourCC>>::into
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:781:9
   5: <four_cc::FourCC as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/git/checkouts/four-cc-2fcc53021f1348b4/c6a750a/src/lib.rs:227:25
   6: repro_four_cc_src_lib_rs_129_17_390afb::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

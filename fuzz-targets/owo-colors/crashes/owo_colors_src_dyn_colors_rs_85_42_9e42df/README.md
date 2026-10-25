# owo-colors/src/dyn_colors.rs:85:42

- crate: owo-colors 4.4.0
- kind: panic
- location: `owo-colors/src/dyn_colors.rs:85:42`
- last API call: `owo_colors::DynColors::from_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15467064028862927834

## Message

```
end byte index 3 is not a char boundary; it is inside '\u{10ffff}' (bytes 1..5 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
owo-colors = { version = "=4.4.0" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <owo_colors::DynColors as ::core::str::FromStr>::from_str("#\u{10ffff}_8");
}
```

Run with `cargo run` in this directory (package `repro_owo_colors_src_dyn_colors_rs_85_42_9e42df`).

## Output

```
thread 'main' (3536537) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/owo-colors-4.4.0/src/dyn_colors.rs:85:42:
end byte index 3 is not a char boundary; it is inside '\u{10ffff}' (bytes 1..5 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: <owo_colors::dyn_colors::DynColors as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/owo-colors-4.4.0/src/dyn_colors.rs:85:42
   8: repro_owo_colors_src_dyn_colors_rs_85_42_9e42df::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

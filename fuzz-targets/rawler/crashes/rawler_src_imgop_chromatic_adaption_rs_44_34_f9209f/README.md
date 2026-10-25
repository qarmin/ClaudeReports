# rawler/src/imgop/chromatic_adaption.rs:44:34

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/imgop/chromatic_adaption.rs:44:34`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/imgop/chromatic_adaption.rs#44
- last API call: `rawler::imgop::chromatic_adaption::bradford_adaption_matrix`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not yet implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::imgop::xyz::Illuminant;

fn main() {
    let _ = rawler::imgop::chromatic_adaption::bradford_adaption_matrix(&Illuminant::A, &Illuminant::CloudyWeather);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_imgop_chromatic_adaption_rs_44_34_f9209f`).

## Output

```
thread 'main' (431443) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/imgop/chromatic_adaption.rs:44:34:
not yet implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: rawler::imgop::chromatic_adaption::illuminant_to_XYZ_tristimulus
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/imgop/chromatic_adaption.rs:44:34
   4: rawler::imgop::chromatic_adaption::bradford_adaption_matrix
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/imgop/chromatic_adaption.rs:64:25
   5: repro_rawler_src_imgop_chromatic_adaption_rs_44_34_f9209f::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

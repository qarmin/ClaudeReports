# object/src/macho.rs:3537:9

- crate: object 0.40.0
- kind: panic
- location: `object/src/macho.rs:3537:9`
- source: https://docs.rs/crate/object/0.40.0/source/src/macho.rs#3537
- last API call: `object::macho::SymbolDesc::with_common_alignment`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 64
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { version = "=0.40.0" }
```

`src/main.rs`:

```rust
use object::macho::N_ALT_ENTRY;

fn main() {
    let _ = N_ALT_ENTRY.with_common_alignment(68u8);
}
```

Run with `cargo run` in this directory (package `repro_object_src_macho_rs_3537_9_e1b41a`).

## Output

```
thread 'main' (2422113) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/macho.rs:3537:9:
assertion `left == right` failed
  left: 64
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<u8, u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <object::macho::SymbolDesc>::with_common_alignment
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/macho.rs:3537:9
   5: repro_object_src_macho_rs_3537_9_e1b41a::main
             at ./src/main.rs:4:25
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

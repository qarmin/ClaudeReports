# object/src/macho.rs:263:5

- crate: object 0.40.0
- kind: panic
- location: `object/src/macho.rs:263:5`
- source: https://docs.rs/crate/object/0.40.0/source/src/macho.rs#263
- last API call: `object::macho::cpu_subtype_intel`
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
object = { version = "=0.40.0" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = object::macho::cpu_subtype_intel(16u32, u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_object_src_macho_rs_263_5_c033b9`).

## Output

```
thread 'main' (2424080) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/macho.rs:263:5:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: object::macho::cpu_subtype_intel
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/macho.rs:263:5
   4: repro_object_src_macho_rs_263_5_c033b9::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

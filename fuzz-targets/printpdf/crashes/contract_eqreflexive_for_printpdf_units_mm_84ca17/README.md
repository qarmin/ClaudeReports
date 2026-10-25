# contract: EqReflexive for printpdf::units::Mm

- crate: printpdf 0.12.8
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `printpdf::units::Mm {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: EqReflexive for printpdf::units::Mm
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8" }
```

`src/main.rs`:

```rust
use printpdf::units::Mm;

fn main() {
    let mm = Mm(f32::NAN);
    assert!(mm == mm, "Eq is not reflexive");
}
```

Run with `cargo run` in this directory (package `repro_contract_eqreflexive_for_printpdf_units_mm_84ca17`).

## Output

```
thread 'main' (253062) panicked at src/main.rs:5:5:
Eq is not reflexive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: repro_contract_eqreflexive_for_printpdf_units_mm_84ca17::main
             at ./src/main.rs:5:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# contract: EqReflexive for krilla::paint::Paint

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `krilla::paint::Paint::from<krilla::paint::LinearGradient>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: EqReflexive for krilla::paint::Paint
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::geom::Transform;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;

fn main() {
    let paint = Paint::from(LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32, y2: 0.0f32, transform: Transform::from_scale(0.0f32, 0.0f32).invert().unwrap(), spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false });
    assert!(paint == paint, "Eq is not reflexive");
}
```

Run with `cargo run` in this directory (package `repro_contract_eqreflexive_for_krilla_paint_paint_eada92`).

## Output

```
thread 'main' (250561) panicked at src/main.rs:8:5:
Eq is not reflexive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: repro_contract_eqreflexive_for_krilla_paint_paint_eada92::main
             at ./src/main.rs:8:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

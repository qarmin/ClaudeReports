# contract: EqReflexive for krilla::paint::Stroke

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `krilla::paint::Stroke {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: EqReflexive for krilla::paint::Stroke
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::color::luma::Color;
use krilla::num::NormalizedF32;
use krilla::paint::LineCap;
use krilla::paint::LineJoin;
use krilla::paint::Paint;
use krilla::paint::Stroke;
use krilla::paint::StrokeDash;

fn main() {
    let stroke = Stroke { paint: Paint::from(Color::new(1u8)), width: 0.0f32, miter_limit: 0.0f32, line_cap: LineCap::Round, line_join: LineJoin::default(), opacity: NormalizedF32::ZERO, dash: Some(StrokeDash { array: vec![f32::NAN], offset: 1.0f32 }) };
    assert!(stroke == stroke, "Eq is not reflexive");
}
```

Run with `cargo run` in this directory (package `repro_contract_eqreflexive_for_krilla_paint_stroke_69ffe4`).

## Output

```
thread 'main' (250257) panicked at src/main.rs:11:5:
Eq is not reflexive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: repro_contract_eqreflexive_for_krilla_paint_stroke_69ffe4::main
             at ./src/main.rs:11:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

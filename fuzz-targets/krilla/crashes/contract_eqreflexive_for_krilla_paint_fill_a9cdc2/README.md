# contract: EqReflexive for krilla::paint::Fill

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `krilla::paint::Fill {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: EqReflexive for krilla::paint::Fill
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
use krilla::num::NormalizedF32;
use krilla::paint::Fill;
use krilla::paint::FillRule;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::SweepGradient;

fn main() {
    let fill = Fill { paint: Paint::from(SweepGradient { cx: 0.0f32, cy: f32::NAN, start_angle: 0.0f32, end_angle: 0.0f32, transform: Transform::identity(), spread_method: SpreadMethod::default(), stops: vec![], anti_alias: false }), opacity: NormalizedF32::ZERO, rule: FillRule::NonZero };
    assert!(fill == fill, "Eq is not reflexive");
}
```

Run with `cargo run` in this directory (package `repro_contract_eqreflexive_for_krilla_paint_fill_a9cdc2`).

## Output

```
thread 'main' (250018) panicked at src/main.rs:11:5:
Eq is not reflexive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: repro_contract_eqreflexive_for_krilla_paint_fill_a9cdc2::main
             at ./src/main.rs:11:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# allsorts/src/layout.rs:3368:29

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/layout.rs:3368:29`
- last API call: `allsorts::layout::SingleSubst::apply_glyph`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 5011606815158414438

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::layout::Coverage;
use allsorts::layout::SingleSubst;

fn main() {
    let single_subst = SingleSubst::Format2 { coverage: std::sync::Arc::new(Coverage::Format2 { coverage_range_array: vec![<allsorts::layout::CoverageRangeRecord as allsorts::binary::read::ReadFrom>::read_from((0u16, 1u16, u16::MAX))] }), substitute_glyph_array: vec![] };
    let _ = single_subst.apply_glyph(1u16);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_layout_rs_3368_29_782fde`).

## Output

```
thread 'main' (81548) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/layout.rs:3368:29:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::layout::Coverage>::glyph_coverage_value
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/layout.rs:3368:29
   4: <allsorts::layout::SingleSubst>::apply_glyph
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/layout.rs:1434:33
   5: repro_allsorts_src_layout_rs_3368_29_782fde::main
             at ./src/main.rs:6:26
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

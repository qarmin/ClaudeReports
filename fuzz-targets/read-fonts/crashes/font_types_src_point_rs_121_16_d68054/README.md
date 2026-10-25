# font-types/src/point.rs:121:16

- crate: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec
- kind: panic
- location: `font-types/src/point.rs:121:16`
- last API call: `font_types::Point<u32>::mul<font_types::Point<u32>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4930871169398748169

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
read-fonts = { git = "https://github.com/googlefonts/fontations", rev = "5c480698f43ab4ff5b304de5a9204c1b5e720aec" }
font-types = { git = "https://github.com/googlefonts/fontations", rev = "5c480698f43ab4ff5b304de5a9204c1b5e720aec" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <font_types::Point<u32> as ::core::ops::Mul<font_types::Point<u32>>>::mul(<font_types::Point<u32>>::new(1000000000u32, 0u32), <font_types::Point<u32>>::broadcast(10000u32));
}
```

Run with `cargo run` in this directory (package `repro_font_types_src_point_rs_121_16_d68054`).

## Output

```
thread 'main' (249306) panicked at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/font-types/src/point.rs:121:16:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u32 as core::ops::arith::Mul>::mul
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:350:45
   4: <font_types::point::Point<u32> as core::ops::arith::Mul>::mul
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/font-types/src/point.rs:121:16
   5: repro_font_types_src_point_rs_121_16_d68054::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

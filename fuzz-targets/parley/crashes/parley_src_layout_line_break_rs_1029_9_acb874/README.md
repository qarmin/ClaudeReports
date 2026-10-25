# parley/src/layout/line_break.rs:1029:9

- crate: https://github.com/linebender/parley @ 7dc686641055e13b21d8a6757d55bf46a95a4e9e
- kind: panic
- location: `parley/src/layout/line_break.rs:1029:9`
- last API call: `parley::Layout<[u8; 4]>::break_all_lines`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3336910972685912046

## Message

```
assertion failed: self.state.layout_max_advance == f32::INFINITY ||
    self.state.line_max_advance - self.state.layout_max_advance < 1.0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
parley = { git = "https://github.com/linebender/parley", rev = "7dc686641055e13b21d8a6757d55bf46a95a4e9e" }
```

`src/main.rs`:

```rust
fn main() {
    let mut layout = <parley::Layout<[u8; 4]>>::new();
    layout.break_all_lines(Some(f32::NEG_INFINITY));
}
```

Run with `cargo run` in this directory (package `repro_parley_src_layout_line_break_rs_1029_9_acb874`).

## Output

```
thread 'main' (634783) panicked at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parley/src/layout/line_break.rs:1029:9:
assertion failed: self.state.layout_max_advance == f32::INFINITY ||
    self.state.line_max_advance - self.state.layout_max_advance < 1.0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <parley::layout::line_break::BreakLines<[u8; 4]>>::break_next
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parley/src/layout/line_break.rs:1029:9
   4: <parley::layout::line_break::BreakLines<[u8; 4]>>::break_remaining
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parley/src/layout/line_break.rs:1435:43
   5: <parley::layout::layout::Layout<[u8; 4]>>::break_all_lines
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parley/src/layout/layout.rs:212:14
   6: repro_parley_src_layout_line_break_rs_1029_9_acb874::main
             at ./src/main.rs:3:12
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

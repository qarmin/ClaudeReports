# harfrust/src/buffer.rs:1003:12

- crate: https://github.com/harfbuzz/harfrust @ facb5efa7f301e2a27cadb3a43dfde93bba3f1f2
- kind: panic
- location: `harfrust/src/buffer.rs:1003:12`
- last API call: `harfrust::Buffer::reverse_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
harfrust = { git = "https://github.com/harfbuzz/harfrust", rev = "facb5efa7f301e2a27cadb3a43dfde93bba3f1f2" }
```

`src/main.rs`:

```rust
use harfrust::Buffer;

fn main() {
    let mut buffer = Buffer::default();
    buffer.reverse_range(10000000000000000000usize, 255usize);
}
```

Run with `cargo run` in this directory (package `repro_harfrust_src_buffer_rs_1003_12_eecec4`).

## Output

```
thread 'main' (1351336) panicked at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/buffer.rs:1003:12:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <harfrust::buffer::Buffer>::reverse_range
             at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/buffer.rs:1003:12
   4: repro_harfrust_src_buffer_rs_1003_12_eecec4::main
             at ./src/main.rs:5:12
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

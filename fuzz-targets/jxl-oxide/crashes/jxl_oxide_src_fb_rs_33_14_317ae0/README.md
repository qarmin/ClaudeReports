# jxl-oxide/src/fb.rs:33:14

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: panic
- location: `jxl-oxide/src/fb.rs:33:14`
- last API call: `jxl_oxide::FrameBuffer::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
framebuffer dimensions overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_oxide::FrameBuffer;

fn main() {
    let _ = FrameBuffer::new(1usize, 2usize, usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_jxl_oxide_src_fb_rs_33_14_317ae0`).

## Output

```
thread 'main' (407440) panicked at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-oxide/src/fb.rs:33:14:
framebuffer dimensions overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<usize>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <jxl_oxide::fb::FrameBuffer>::buffer_len
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-oxide/src/fb.rs:33:14
   6: <jxl_oxide::fb::FrameBuffer>::new
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-oxide/src/fb.rs:19:19
   7: repro_jxl_oxide_src_fb_rs_33_14_317ae0::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# jxl-render/src/region.rs:204:19

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: panic
- location: `jxl-render/src/region.rs:204:19`
- last API call: `jxl_render::Region::upsample`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-render = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_render::Region;

fn main() {
    let region = Region::with_size(0u32, 1u32);
    let region2 = region.translate(0i32, 0i32);
    let _ = region2.upsample(1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_jxl_render_src_region_rs_204_19_e4526e`).

## Output

```
thread 'main' (407494) panicked at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-render/src/region.rs:204:19:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <jxl_render::region::Region>::upsample_separate
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-render/src/region.rs:204:19
   4: <jxl_render::region::Region>::upsample
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-render/src/region.rs:198:14
   5: repro_jxl_render_src_region_rs_204_19_e4526e::main
             at ./src/main.rs:6:21
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

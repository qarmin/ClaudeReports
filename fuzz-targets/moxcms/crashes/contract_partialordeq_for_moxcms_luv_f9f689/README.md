# contract: PartialOrdEq for moxcms::Luv

- crate: moxcms 0.9.1
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `moxcms::Luv {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: PartialOrdEq for moxcms::Luv
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { version = "=0.9.1" }
```

`src/main.rs`:

```rust
use moxcms::Luv;
use moxcms::Matrix4f;

fn main() {
    let matrix4f = Matrix4f { v: [[0.0f32, 0.0f32, 0.0f32, 0.0f32], [f32::MAX, 0.0f32, 0.0f32, 1.0f32], [0.0f32, 1.0f32, 0.0f32, 57.0f32], [0.0f32, 1.0f32, 1.0f32, 1.0f32]] };
    let f32_v = matrix4f.determinant().unwrap();
    let luv = Luv { l: 0.0f32, u: f32_v, v: 0.0f32 };
    assert_eq!(luv == luv, luv.partial_cmp(&luv) == Some(std::cmp::Ordering::Equal), "PartialOrd inconsistent with PartialEq");
}
```

Run with `cargo run` in this directory (package `repro_contract_partialordeq_for_moxcms_luv_f9f689`).

## Output

```
thread 'main' (2512743) panicked at src/main.rs:8:5:
assertion `left == right` failed: PartialOrd inconsistent with PartialEq
  left: true
 right: false
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<bool, bool>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: repro_contract_partialordeq_for_moxcms_luv_f9f689::main
             at ./src/main.rs:8:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

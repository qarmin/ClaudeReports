# xkeysym/src/lib.rs:304:17

- crate: https://github.com/notgull/xkeysym @ 77596eb8319f8d7a97db6477f307a94a0905c439
- kind: panic
- location: `xkeysym/src/lib.rs:304:17`
- last API call: `xkeysym::keysym`
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
xkeysym = { git = "https://github.com/notgull/xkeysym", rev = "77596eb8319f8d7a97db6477f307a94a0905c439" }
```

`src/main.rs`:

```rust
use xkeysym::KeyCode;

fn main() {
    let key_code = KeyCode::default();
    let _ = xkeysym::keysym(key_code, 0u8, KeyCode::from(1u32), 0u8, &[]);
}
```

Run with `cargo run` in this directory (package `repro_xkeysym_src_lib_rs_304_17_50481c`).

## Output

```
thread 'main' (565359) panicked at /home/rafal/.cargo/git/checkouts/xkeysym-34101e391601194e/77596eb/src/lib.rs:304:17:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: xkeysym::keysym
             at /home/rafal/.cargo/git/checkouts/xkeysym-34101e391601194e/77596eb/src/lib.rs:304:17
   4: repro_xkeysym_src_lib_rs_304_17_50481c::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

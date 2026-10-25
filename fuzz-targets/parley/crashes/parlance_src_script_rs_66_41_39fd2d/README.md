# parlance/src/script.rs:66:41

- crate: https://github.com/linebender/parley @ 7dc686641055e13b21d8a6757d55bf46a95a4e9e
- kind: panic
- location: `parlance/src/script.rs:66:41`
- last API call: `parlance::Script::as_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
only ASCII: Utf8Error { valid_up_to: 0, error_len: Some(1) }
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
parley = { git = "https://github.com/linebender/parley", rev = "7dc686641055e13b21d8a6757d55bf46a95a4e9e" }
parlance = { git = "https://github.com/linebender/parley", rev = "7dc686641055e13b21d8a6757d55bf46a95a4e9e" }
```

`src/main.rs`:

```rust
use parlance::Script;

fn main() {
    let script = Script::from_bytes([255u8, 0u8, 0u8, 0u8]);
    let _ = script.as_str();
}
```

Run with `cargo run` in this directory (package `repro_parlance_src_script_rs_66_41_39fd2d`).

## Output

```
thread 'main' (479232) panicked at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parlance/src/script.rs:66:41:
only ASCII: Utf8Error { valid_up_to: 0, error_len: Some(1) }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<&str, core::str::error::Utf8Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <parlance::script::Script>::as_str
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parlance/src/script.rs:66:41
   5: repro_parlance_src_script_rs_66_41_39fd2d::main
             at ./src/main.rs:5:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

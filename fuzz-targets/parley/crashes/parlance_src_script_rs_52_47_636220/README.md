# parlance/src/script.rs:52:47

- crate: https://github.com/linebender/parley @ 7dc686641055e13b21d8a6757d55bf46a95a4e9e
- kind: panic
- location: `parlance/src/script.rs:52:47`
- last API call: `parlance::Script::from_str_unchecked`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 2 but the index is 2
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
    let _ = Script::from_str_unchecked("v6");
}
```

Run with `cargo run` in this directory (package `repro_parlance_src_script_rs_52_47_636220`).

## Output

```
thread 'main' (479306) panicked at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parlance/src/script.rs:52:47:
index out of bounds: the len is 2 but the index is 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <parlance::script::Script>::from_str_unchecked
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parlance/src/script.rs:52:47
   4: repro_parlance_src_script_rs_52_47_636220::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

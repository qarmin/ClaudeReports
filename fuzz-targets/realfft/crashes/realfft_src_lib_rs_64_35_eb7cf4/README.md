# realfft/src/lib.rs:64:35

- crate: https://github.com/HEnquist/realfft @ d0d4eee0525fd27c96c8a046d6d107acd5ed84a6
- kind: panic
- location: `realfft/src/lib.rs:64:35`
- last API call: `realfft::FftError::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
realfft = { git = "https://github.com/HEnquist/realfft", rev = "d0d4eee0525fd27c96c8a046d6d107acd5ed84a6" }
```

`src/main.rs`:

```rust
use realfft::FftError;

fn main() {
    let fft_error = FftError::InputValues(false, false);
    let _ = fft_error.to_string();
}
```

Run with `cargo run` in this directory (package `repro_realfft_src_lib_rs_64_35_eb7cf4`).

## Output

```
thread 'main' (575631) panicked at /home/rafal/.cargo/git/checkouts/realfft-04d2b093166892fd/d0d4eee/src/lib.rs:64:35:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <realfft::FftError>::fmt_internal
             at /home/rafal/.cargo/git/checkouts/realfft-04d2b093166892fd/d0d4eee/src/lib.rs:64:35
   4: <realfft::FftError as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/realfft-04d2b093166892fd/d0d4eee/src/lib.rs:79:14
   5: <realfft::FftError as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   6: <realfft::FftError as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   7: repro_realfft_src_lib_rs_64_35_eb7cf4::main
             at ./src/main.rs:5:23
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# av1-grain/src/create.rs:373:44

- crate: https://github.com/rust-av/av1-grain @ 3af392b40ddb4973ee6a0ae916d3c7e21222c400
- kind: panic
- location: `av1-grain/src/create.rs:373:44`
- last API call: `av1_grain::generate_photon_noise_params`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
av1-grain = { git = "https://github.com/rust-av/av1-grain", rev = "3af392b40ddb4973ee6a0ae916d3c7e21222c400" }
```

`src/main.rs`:

```rust
use av1_grain::NoiseGenArgs;
use av1_grain::TransferFunction;

fn main() {
    let _ = av1_grain::generate_photon_noise_params(0u64, 0u64, NoiseGenArgs { iso_setting: 0u32, width: 1000u32, height: 100000000u32, transfer_function: TransferFunction::BT1886, full_range: false, chroma_grain: false, random_seed: None });
}
```

Run with `cargo run` in this directory (package `repro_av1_grain_src_create_rs_373_44_325ce5`).

## Output

```
thread 'main' (600785) panicked at /home/rafal/.cargo/git/checkouts/av1-grain-0dec80ef69318a04/3af392b/src/create.rs:373:44:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: av1_grain::create::generate_luma_noise_points
             at /home/rafal/.cargo/git/checkouts/av1-grain-0dec80ef69318a04/3af392b/src/create.rs:373:44
   4: av1_grain::create::generate_photon_noise_params
             at /home/rafal/.cargo/git/checkouts/av1-grain-0dec80ef69318a04/3af392b/src/create.rs:140:27
   5: repro_av1_grain_src_create_rs_373_44_325ce5::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# vsimd/src/table.rs:20:5

- crate: vsimd 0.8.0
- kind: panic
- location: `vsimd/src/table.rs:20:5`
- source: https://docs.rs/crate/vsimd/0.8.0/source/src/table.rs#20
- last API call: `vsimd::hex::decode_ascii32[SSE2]`
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
vsimd = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let _ = vsimd::hex::decode_ascii32::<vsimd::isa::SSE2>(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), ENCODE_LOWER_LUT);
}
```

Run with `cargo run` in this directory (package `repro_vsimd_src_table_rs_20_5_c89f69`).

## Output

```
thread 'main' (2570243) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vsimd-0.8.0/src/table.rs:20:5:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: vsimd::table::u8x16xn_lookup::<vsimd::isa::SSE2, vsimd::vector::V256>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vsimd-0.8.0/src/table.rs:20:5
   4: vsimd::alsw::decode_ascii_xn::<vsimd::isa::SSE2, vsimd::vector::V256>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vsimd-0.8.0/src/alsw.rs:65:32
   5: vsimd::hex::decode32::<vsimd::isa::SSE2>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vsimd-0.8.0/src/hex.rs:140:20
   6: vsimd::hex::decode_ascii32::<vsimd::isa::SSE2>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vsimd-0.8.0/src/hex.rs:169:27
   7: repro_vsimd_src_table_rs_20_5_c89f69::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

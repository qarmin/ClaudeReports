# flate2-rs/src/mem.rs:338:22

- crate: https://github.com/rust-lang/flate2-rs @ 31d6aef2a1478b61d8aeb5931d74f34761add25a
- kind: panic
- location: `flate2-rs/src/mem.rs:338:22`
- last API call: `flate2::Compress::set_level`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unknown return code: -2
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
flate2 = { git = "https://github.com/rust-lang/flate2-rs", rev = "31d6aef2a1478b61d8aeb5931d74f34761add25a", features = ["any_c_zlib", "any_impl", "any_zlib", "cloudflare_zlib", "document-features", "libz-ng-sys", "libz-sys", "miniz-sys", "miniz_oxide", "runtime_detection", "rust_backend", "zlib", "zlib-default", "zlib-ng", "zlib-ng-compat", "zlib-rs"] }
```

`src/main.rs`:

```rust
use flate2::Compress;
use flate2::Compression;

fn main() {
    let mut compress = Compress::new(Compression::default(), false);
    let _ = compress.set_level(Compression::new(1000000000u32));
}
```

Run with `cargo run` in this directory (package `repro_flate2_rs_src_mem_rs_338_22_bd4a40`).

## Output

```
thread 'main' (1414745) panicked at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/mem.rs:338:22:
unknown return code: -2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <flate2::mem::Compress>::set_level
             at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/mem.rs:338:22
   3: repro_flate2_rs_src_mem_rs_338_22_bd4a40::main
             at ./src/main.rs:6:22
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

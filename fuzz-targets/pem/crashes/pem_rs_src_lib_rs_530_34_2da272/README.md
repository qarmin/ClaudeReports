# pem-rs/src/lib.rs:530:34

- crate: https://github.com/jcreekmore/pem-rs @ 99c15c08f1389153ed037c33750f8605bbf8bd55
- kind: panic
- location: `pem-rs/src/lib.rs:530:34`
- last API call: `pem::encode_config`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
chunk size must be non-zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
pem = { git = "https://github.com/jcreekmore/pem-rs", rev = "99c15c08f1389153ed037c33750f8605bbf8bd55" }
```

`src/main.rs`:

```rust
use pem::EncodeConfig;
use pem::Pem;

fn main() {
    let _ = pem::encode_config(&Pem::new("".to_string(), vec![]), EncodeConfig::new().set_line_wrap(0usize));
}
```

Run with `cargo run` in this directory (package `repro_pem_rs_src_lib_rs_530_34_2da272`).

## Output

```
thread 'main' (1430914) panicked at /home/rafal/.cargo/git/checkouts/pem-rs-c5170bb56e389ab0/99c15c0/src/lib.rs:530:34:
chunk size must be non-zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <[u8]>::chunks
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1155:9
   3: pem::encode_config
             at /home/rafal/.cargo/git/checkouts/pem-rs-c5170bb56e389ab0/99c15c0/src/lib.rs:530:34
   4: repro_pem_rs_src_lib_rs_530_34_2da272::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

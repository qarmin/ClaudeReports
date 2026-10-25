# asn1-rs/src/asn1_types/real.rs:390:21

- crate: https://github.com/rusticata/asn1-rs @ 6fb39fb14ba71cf1741e5ea5c69e43f9a85fb33f
- kind: panic
- location: `asn1-rs/src/asn1_types/real.rs:390:21`
- last API call: `asn1_rs::Real::der_content_len`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: m != 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
asn1-rs = { git = "https://github.com/rusticata/asn1-rs", rev = "6fb39fb14ba71cf1741e5ea5c69e43f9a85fb33f" }
```

`src/main.rs`:

```rust
use asn1_rs::Real;

fn main() {
    let _ = <asn1_rs::Real as asn1_rs::ToDer>::der_content_len(&Real::binary(-0.0f64, 2u32, 0i32));
}
```

Run with `cargo run` in this directory (package `repro_asn1_rs_src_asn1_types_real_rs_390_21_990ffd`).

## Output

```
thread 'main' (1458504) panicked at /home/rafal/.cargo/git/checkouts/asn1-rs-504720fdb1757e8f/6fb39fb/src/asn1_types/real.rs:390:21:
assertion failed: m != 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <asn1_rs::asn1_types::real::Real as asn1_rs::to_ber::ToBer>::ber_write_content::<core::io::util::Sink>
             at /home/rafal/.cargo/git/checkouts/asn1-rs-504720fdb1757e8f/6fb39fb/src/asn1_types/real.rs:390:21
   4: <asn1_rs::asn1_types::real::Real as asn1_rs::to_ber::ToBer>::ber_content_len
             at /home/rafal/.cargo/git/checkouts/asn1-rs-504720fdb1757e8f/6fb39fb/src/asn1_types/real.rs:348:34
   5: <asn1_rs::asn1_types::real::Real as asn1_rs::to_der::ToDer>::der_content_len
             at /home/rafal/.cargo/git/checkouts/asn1-rs-504720fdb1757e8f/6fb39fb/src/to_der.rs:195:17
   6: repro_asn1_rs_src_asn1_types_real_rs_390_21_990ffd::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

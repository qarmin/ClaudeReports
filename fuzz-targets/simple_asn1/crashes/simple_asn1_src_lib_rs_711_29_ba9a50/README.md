# simple_asn1/src/lib.rs:711:29

- crate: https://github.com/acw/simple_asn1 @ 7712c7cb4d72e52b6896ba56990ed1095a317de5
- kind: panic
- location: `simple_asn1/src/lib.rs:711:29`
- last API call: `simple_asn1::to_der`
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
simple_asn1 = { git = "https://github.com/acw/simple_asn1", rev = "7712c7cb4d72e52b6896ba56990ed1095a317de5" }
```

`src/main.rs`:

```rust
use simple_asn1::ASN1Block;

fn main() {
    let a_s_n1_block = ASN1Block::BitString(0usize, 1usize, vec![]);
    let _ = simple_asn1::to_der(&a_s_n1_block);
}
```

Run with `cargo run` in this directory (package `repro_simple_asn1_src_lib_rs_711_29_ba9a50`).

## Output

```
thread 'main' (572389) panicked at /home/rafal/.cargo/git/checkouts/simple_asn1-92132e2a0a0fca50/7712c7c/src/lib.rs:711:29:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: simple_asn1::to_der
             at /home/rafal/.cargo/git/checkouts/simple_asn1-92132e2a0a0fca50/7712c7c/src/lib.rs:711:29
   4: repro_simple_asn1_src_lib_rs_711_29_ba9a50::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

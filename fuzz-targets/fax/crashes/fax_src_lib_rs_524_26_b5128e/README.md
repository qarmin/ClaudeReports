# fax/src/lib.rs:524:26

- crate: https://github.com/pdf-rs/fax @ 8f26f4f63a54fb93529ffdedd9b5a584c3ed01af
- kind: panic
- location: `fax/src/lib.rs:524:26`
- last API call: `fax::Bits::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fax = { git = "https://github.com/pdf-rs/fax", rev = "8f26f4f63a54fb93529ffdedd9b5a584c3ed01af" }
```

`src/main.rs`:

```rust
use fax::Bits;

fn main() {
    let bits = Bits { data: 0u16, len: 26u8 };
    let _ = bits.to_string();
}
```

Run with `cargo run` in this directory (package `repro_fax_src_lib_rs_524_26_b5128e`).

## Output

```
thread 'main' (478889) panicked at /home/rafal/.cargo/git/checkouts/fax-3c34a03a0b7cfe42/8f26f4f/src/lib.rs:524:26:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <fax::Bits as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/fax-3c34a03a0b7cfe42/8f26f4f/src/lib.rs:524:26
   4: <fax::Bits as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   5: <fax::Bits as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   6: repro_fax_src_lib_rs_524_26_b5128e::main
             at ./src/main.rs:5:18
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

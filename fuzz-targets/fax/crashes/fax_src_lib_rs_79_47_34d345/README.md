# fax/src/lib.rs:79:47

- crate: https://github.com/pdf-rs/fax @ 8f26f4f63a54fb93529ffdedd9b5a584c3ed01af
- kind: panic
- location: `fax/src/lib.rs:79:47`
- last API call: `fax::VecWriter::write`
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
fax = { git = "https://github.com/pdf-rs/fax", rev = "8f26f4f63a54fb93529ffdedd9b5a584c3ed01af" }
```

`src/main.rs`:

```rust
use fax::Bits;
use fax::VecWriter;

fn main() {
    let bits = Bits { data: 0u16, len: 100u8 };
    let mut vec_writer = VecWriter::new();
    let _ = <fax::VecWriter as fax::BitWriter>::write(&mut vec_writer, bits);
}
```

Run with `cargo run` in this directory (package `repro_fax_src_lib_rs_79_47_34d345`).

## Output

```
thread 'main' (479010) panicked at /home/rafal/.cargo/git/checkouts/fax-3c34a03a0b7cfe42/8f26f4f/src/lib.rs:79:47:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <fax::VecWriter as fax::BitWriter>::write
             at /home/rafal/.cargo/git/checkouts/fax-3c34a03a0b7cfe42/8f26f4f/src/lib.rs:79:47
   4: repro_fax_src_lib_rs_79_47_34d345::main
             at ./src/main.rs:7:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

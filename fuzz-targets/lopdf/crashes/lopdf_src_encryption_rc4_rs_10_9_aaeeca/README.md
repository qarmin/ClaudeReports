# lopdf/src/encryption/rc4.rs:10:9

- crate: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1
- kind: panic
- location: `lopdf/src/encryption/rc4.rs:10:9`
- last API call: `lopdf::encryption::crypt_filters::Rc4CryptFilter::decrypt`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !key.is_empty() && key.len() <= 256
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lopdf = { git = "https://github.com/J-F-Liu/lopdf", rev = "bebffeff2d9e54b00325014a4fda07510eac2cb1" }
```

`src/main.rs`:

```rust
use lopdf::encryption::crypt_filters::Rc4CryptFilter;

fn main() {
    let _ = <lopdf::encryption::crypt_filters::Rc4CryptFilter as lopdf::encryption::crypt_filters::CryptFilter>::decrypt(&Rc4CryptFilter, &vec![], &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_lopdf_src_encryption_rc4_rs_10_9_aaeeca`).

## Output

```
thread 'main' (395802) panicked at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/encryption/rc4.rs:10:9:
assertion failed: !key.is_empty() && key.len() <= 256
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <lopdf::encryption::rc4::Rc4>::new::<&[u8]>
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/encryption/rc4.rs:10:9
   4: <lopdf::encryption::crypt_filters::Rc4CryptFilter as lopdf::encryption::crypt_filters::CryptFilter>::decrypt
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/encryption/crypt_filters.rs:78:12
   5: repro_lopdf_src_encryption_rc4_rs_10_9_aaeeca::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

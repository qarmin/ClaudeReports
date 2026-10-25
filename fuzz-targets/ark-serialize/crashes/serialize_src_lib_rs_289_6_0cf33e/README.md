# serialize/src/lib.rs:289:6

- crate: https://github.com/arkworks-rs/algebra @ 4a19a1c50606ae4a0e96ccce107f6281aa0be38e
- kind: panic
- location: `serialize/src/lib.rs:289:6`
- last API call: `ark_serialize::buffer_bit_byte_size`
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
ark-serialize = { git = "https://github.com/arkworks-rs/algebra", rev = "4a19a1c50606ae4a0e96ccce107f6281aa0be38e" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = ark_serialize::buffer_bit_byte_size(usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_serialize_src_lib_rs_289_6_0cf33e`).

## Output

```
thread 'main' (1460095) panicked at /home/rafal/.cargo/git/checkouts/algebra-5142117e1244ae01/4a19a1c/serialize/src/lib.rs:289:6:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: ark_serialize::buffer_bit_byte_size
             at /home/rafal/.cargo/git/checkouts/algebra-5142117e1244ae01/4a19a1c/serialize/src/lib.rs:289:6
   4: repro_serialize_src_lib_rs_289_6_0cf33e::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

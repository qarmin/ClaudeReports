# byteorder/src/lib.rs:1994:9

- crate: https://github.com/BurntSushi/byteorder @ 5a82625fae462e8ba64cec8146b24a372b4d75c6
- kind: panic
- location: `byteorder/src/lib.rs:1994:9`
- last API call: `byteorder::BigEndian::read_u128_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 1
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
byteorder = { git = "https://github.com/BurntSushi/byteorder", rev = "5a82625fae462e8ba64cec8146b24a372b4d75c6" }
```

`src/main.rs`:

```rust
fn main() {
    <byteorder::BigEndian as byteorder::ByteOrder>::read_u128_into(&vec![66u8], &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_byteorder_src_lib_rs_1994_9_ed54ff`).

## Output

```
thread 'main' (1256367) panicked at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:1994:9:
assertion `left == right` failed
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <byteorder::BigEndian as byteorder::ByteOrder>::read_u128_into
             at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:1866:9
   5: repro_byteorder_src_lib_rs_1994_9_ed54ff::main
             at ./src/main.rs:2:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

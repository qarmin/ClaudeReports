# byteorder/src/lib.rs:2090:32

- crate: https://github.com/BurntSushi/byteorder @ 5a82625fae462e8ba64cec8146b24a372b4d75c6
- kind: panic
- location: `byteorder/src/lib.rs:2090:32`
- last API call: `byteorder::LittleEndian::read_u128`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 16 out of range for slice of length 0
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
    let _ = <byteorder::LittleEndian as byteorder::ByteOrder>::read_u128(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_byteorder_src_lib_rs_2090_32_024237`).

## Output

```
thread 'main' (1256240) panicked at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:2090:32:
range end index 16 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:503:23
   5: <[u8] as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   6: <byteorder::LittleEndian as byteorder::ByteOrder>::read_u128
             at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:2090:32
   7: repro_byteorder_src_lib_rs_2090_32_024237::main
             at ./src/main.rs:2:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

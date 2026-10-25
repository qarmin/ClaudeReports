# allsorts/src/binary/write.rs:316:19

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/binary/write.rs:316:19`
- last API call: `allsorts::binary::write::WriteBuffer::write_zeros`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 5923677349985158175

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::binary::write::WriteBuffer;

fn main() {
    let mut write_buffer = WriteBuffer::new();
    let _ = <allsorts::binary::write::WriteBuffer as allsorts::binary::write::WriteContext>::write_zeros(&mut write_buffer, 10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_binary_write_rs_316_19_dd059a`).

## Output

```
thread 'main' (111975) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   5: <alloc::vec::Vec<u8>>::extend_trusted::<core::iter::sources::repeat_n::RepeatN<u8>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4128:18
   6: <alloc::vec::Vec<u8> as alloc::vec::spec_extend::SpecExtend<u8, core::iter::sources::repeat_n::RepeatN<u8>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:27:14
   7: <alloc::vec::Vec<u8> as core::iter::traits::collect::Extend<u8>>::extend::<core::iter::sources::repeat_n::RepeatN<u8>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4064:9
   8: <allsorts::binary::write::WriteBuffer as allsorts::binary::write::WriteContext>::write_zeros
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:316:19
   9: repro_allsorts_src_binary_write_rs_316_19_dd059a::main
             at ./src/main.rs:5:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

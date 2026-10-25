# out of memory in allsorts::binary::write::WriteBuffer::write_zeros

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: out of memory
- last API call: `allsorts::binary::write::WriteBuffer::write_zeros`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14109984585535999572

## Message

```
process ended abnormally (out of memory) while calling allsorts::binary::write::WriteBuffer::write_zeros
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
    let _ = <allsorts::binary::write::WriteBuffer as allsorts::binary::write::WriteContext>::write_zeros(&mut write_buffer, 2147483648usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_allsorts_binary_write_writebuffer_write_zer_28ace7`).

## Output

```
memory allocation of 2147483648 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::vec::Vec<u8>>::extend_trusted::<core::iter::sources::repeat_n::RepeatN<u8>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4128:18
   7: <alloc::vec::Vec<u8> as alloc::vec::spec_extend::SpecExtend<u8, core::iter::sources::repeat_n::RepeatN<u8>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:27:14
   8: <alloc::vec::Vec<u8> as core::iter::traits::collect::Extend<u8>>::extend::<core::iter::sources::repeat_n::RepeatN<u8>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4064:9
   9: <allsorts::binary::write::WriteBuffer as allsorts::binary::write::WriteContext>::write_zeros
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:316:19
  10: repro_out_of_memory_in_allsorts_binary_write_writebuffer_write_zer_28ace7::main
             at ./src/main.rs:5:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

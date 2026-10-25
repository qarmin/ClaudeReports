# object/src/elf.rs:2553:9

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/elf.rs:2553:9`
- last API call: `object::elf::VersionIndex::sub<object::elf::VersionIndex>`
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
object = { git = "https://github.com/gimli-rs/object", rev = "4ef27e429dd86949ac3d3a995b3266440bedcf5c" }
```

`src/main.rs`:

```rust
use object::elf::VersionIndex;

fn main() {
    let _ = <object::elf::VersionIndex as ::core::ops::Sub<object::elf::VersionIndex>>::sub(VersionIndex(0u16), VersionIndex::default().offset(1u16));
}
```

Run with `cargo run` in this directory (package `repro_object_src_elf_rs_2553_9_3fe0fb`).

## Output

```
thread 'main' (1242145) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/elf.rs:2553:9:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <object::elf::VersionIndex>::offset_from
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/elf.rs:2553:9
   4: <object::elf::VersionIndex as core::ops::arith::Sub>::sub
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/elf.rs:2600:14
   5: repro_object_src_elf_rs_2553_9_3fe0fb::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

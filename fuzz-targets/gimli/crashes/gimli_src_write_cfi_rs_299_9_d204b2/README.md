# gimli/src/write/cfi.rs:299:9

- crate: https://github.com/gimli-rs/gimli @ 64a7a194fb5fe43d474fd9c101555e5a81b1d36c
- kind: panic
- location: `gimli/src/write/cfi.rs:299:9`
- last API call: `gimli::write::FrameDescriptionEntry::add_instruction`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.instructions.last().map(|x| x.0).unwrap_or(0) <= offset
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gimli = { git = "https://github.com/gimli-rs/gimli", rev = "64a7a194fb5fe43d474fd9c101555e5a81b1d36c" }
```

`src/main.rs`:

```rust
use gimli::RiscV;
use gimli::write::Address;
use gimli::write::CallFrameInstruction;
use gimli::write::FrameDescriptionEntry;

fn main() {
    let call_frame_instruction = CallFrameInstruction::Restore(RiscV::X28);
    let mut frame_description_entry = FrameDescriptionEntry::new(Address::Constant(0u64), 0u32);
    frame_description_entry.add_instruction(1u32, CallFrameInstruction::RestoreState);
    frame_description_entry.add_instruction(0u32, call_frame_instruction.clone());
}
```

Run with `cargo run` in this directory (package `repro_gimli_src_write_cfi_rs_299_9_d204b2`).

## Output

```
thread 'main' (431153) panicked at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/cfi.rs:299:9:
assertion failed: self.instructions.last().map(|x| x.0).unwrap_or(0) <= offset
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <gimli::write::cfi::FrameDescriptionEntry>::add_instruction
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/cfi.rs:299:9
   4: repro_gimli_src_write_cfi_rs_299_9_d204b2::main
             at ./src/main.rs:10:29
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

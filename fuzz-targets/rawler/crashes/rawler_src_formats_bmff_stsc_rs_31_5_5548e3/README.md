# rawler/src/formats/bmff/stsc.rs:31:5

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/formats/bmff/stsc.rs:31:5`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/formats/bmff/stsc.rs#31
- last API call: `rawler::formats::bmff::stsc::StscBox::get_entry_for_sample`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: stsc box must contains at least one entry
  left: true
 right: false
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::formats::bmff::BoxHeader;
use rawler::formats::bmff::stsc::StscBox;

fn main() {
    let stsc_box = StscBox { header: BoxHeader::default(), version: 0u8, flags: 0u32, entries: vec![] };
    let _ = stsc_box.get_entry_for_sample(1u32);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_formats_bmff_stsc_rs_31_5_5548e3`).

## Output

```
thread 'main' (431724) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/formats/bmff/stsc.rs:31:5:
assertion `left == right` failed: stsc box must contains at least one entry
  left: true
 right: false
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<bool, bool>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <rawler::formats::bmff::stsc::StscBox>::get_entry_for_sample
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/formats/bmff/stsc.rs:31:5
   5: repro_rawler_src_formats_bmff_stsc_rs_31_5_5548e3::main
             at ./src/main.rs:6:22
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

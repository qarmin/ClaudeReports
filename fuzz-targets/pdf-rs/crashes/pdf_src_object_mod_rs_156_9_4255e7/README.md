# pdf/src/object/mod.rs:156:9

- crate: https://github.com/pdf-rs/pdf @ 333ec00750b0943f2287e1775c5bdbfb80a6a545
- kind: panic
- location: `pdf/src/object/mod.rs:156:9`
- last API call: `pdf::object::NoUpdate::update_with[LabParams]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
explicit panic
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
pdf = { git = "https://github.com/pdf-rs/pdf", rev = "333ec00750b0943f2287e1775c5bdbfb80a6a545" }
```

`src/main.rs`:

```rust
use pdf::file::UpdateOptions;
use pdf::object::LabParams;
use pdf::object::NoUpdate;

fn main() {
    let mut no_update = NoUpdate;
    let _ = <pdf::object::NoUpdate as pdf::object::Updater>::update_with::<pdf::object::LabParams>(&mut no_update, <pdf::object::Ref<pdf::object::Stream<()>>>::from_id(0u64).get_inner(), LabParams { white_point: vec![], black_point: None, range: None }, UpdateOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_pdf_src_object_mod_rs_156_9_4255e7`).

## Output

```
thread 'main' (3413581) panicked at /home/rafal/.cargo/git/checkouts/pdf-9563b332f7e1af00/333ec00/pdf/src/object/mod.rs:156:9:
explicit panic
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <pdf::object::NoUpdate as pdf::object::Updater>::update_with::<pdf::object::color::LabParams>
             at /home/rafal/.cargo/git/checkouts/pdf-9563b332f7e1af00/333ec00/pdf/src/object/mod.rs:156:9
   4: repro_pdf_src_object_mod_rs_156_9_4255e7::main
             at ./src/main.rs:7:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# jiff/src/tz/offset.rs:204:21

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/tz/offset.rs:204:21`
- last API call: `jiff::tz::Offset::to_datetime`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
invalid time zone offset hours
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::fmt::strtime::BrokenDownTime;
use jiff::tz::Offset;

fn main() {
    let _ = Offset::constant(100i8).to_datetime(BrokenDownTime::parse(vec![127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8], include_bytes!("image_0.tiff").to_vec()).unwrap().timestamp().unwrap());
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_tz_offset_rs_204_21_44427e`).

## Output

```
thread 'main' (361044) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/tz/offset.rs:204:21:
invalid time zone offset hours
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <jiff::tz::offset::Offset>::constant
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/util/constant.rs:20:23
   3: repro_jiff_src_tz_offset_rs_204_21_44427e::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

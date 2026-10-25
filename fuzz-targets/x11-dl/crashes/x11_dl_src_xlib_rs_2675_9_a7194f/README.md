# x11-dl/src/xlib.rs:2675:9

- crate: https://github.com/AltF02/x11-rs @ ff57a942e5a4d35d8660362a8ca2ac0c02fa4852
- kind: panic
- location: `x11-dl/src/xlib.rs:2675:9`
- last API call: `x11_dl::xlib::ClientMessageData::get_byte`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 20 but the index is 1000000000
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
x11-dl = { git = "https://github.com/AltF02/x11-rs", rev = "ff57a942e5a4d35d8660362a8ca2ac0c02fa4852" }
```

`src/main.rs`:

```rust
use x11_dl::xlib::ClientMessageData;

fn main() {
    let client_message_data = ClientMessageData::from([1u64, 0u64, 0u64, 0u64, 1u64]);
    let _ = client_message_data.get_byte(1000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_x11_dl_src_xlib_rs_2675_9_a7194f`).

## Output

```
thread 'main' (523012) panicked at /home/rafal/.cargo/git/checkouts/x11-rs-36eb3ccc656745ea/ff57a94/x11-dl/src/xlib.rs:2675:9:
index out of bounds: the len is 20 but the index is 1000000000
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <x11_dl::xlib::ClientMessageData>::get_byte
             at /home/rafal/.cargo/git/checkouts/x11-rs-36eb3ccc656745ea/ff57a94/x11-dl/src/xlib.rs:2675:9
   4: repro_x11_dl_src_xlib_rs_2675_9_a7194f::main
             at ./src/main.rs:5:33
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

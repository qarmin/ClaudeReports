# unicode-bidi/src/implicit.rs:596:36

- crate: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad
- kind: panic
- location: `unicode-bidi/src/implicit.rs:596:36`
- last API call: `unicode_bidi::BidiInfo::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Level number error: OutOfRangeNumber
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
unicode-bidi = { git = "https://github.com/servo/unicode-bidi", rev = "9a8c7b03be01def5cf7014b89d5af90f7382fdad" }
```

`src/main.rs`:

```rust
use unicode_bidi::BidiInfo;
use unicode_bidi::Level;

fn main() {
    let u8_v = Level::max_implicit_depth();
    let string = "ﰍ".to_string();
    let _ = BidiInfo::new(&string, Some(Level::from(u8_v)));
}
```

Run with `cargo run` in this directory (package `repro_unicode_bidi_src_implicit_rs_596_36_7e33d9`).

## Output

```
thread 'main' (1245872) panicked at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/implicit.rs:596:36:
Level number error: OutOfRangeNumber
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), unicode_bidi::level::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: unicode_bidi::implicit::resolve_levels
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/implicit.rs:596:36
   5: unicode_bidi::compute_bidi_info_for_para::<unicode_bidi::char_data::HardcodedBidiData, str>
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:1153:5
   6: <unicode_bidi::BidiInfo>::new_with_data_source::<unicode_bidi::char_data::HardcodedBidiData>
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:512:13
   7: <unicode_bidi::BidiInfo>::new
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:485:9
   8: repro_unicode_bidi_src_implicit_rs_596_36_7e33d9::main
             at ./src/main.rs:7:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

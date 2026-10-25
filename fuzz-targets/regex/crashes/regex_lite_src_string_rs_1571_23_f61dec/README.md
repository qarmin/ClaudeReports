# regex-lite/src/string.rs:1571:23

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-lite/src/string.rs:1571:23`
- last API call: `regex_lite::Match::as_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4913380211630276896

## Message

```
start byte index 1 is not a char boundary; it is inside '\u{c0aa7}' (bytes 0..4 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-lite = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
```

`src/main.rs`:

```rust
use regex_lite::Regex;

fn main() {
    let regex = Regex::try_from("".to_string()).unwrap();
    let string = "\u{c0aa7}".to_string();
    let match_v = regex.find_at(&string, 1usize).unwrap();
    let _ = match_v.as_str();
}
```

Run with `cargo run` in this directory (package `repro_regex_lite_src_string_rs_1571_23_f61dec`).

## Output

```
thread 'main' (359174) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-lite/src/string.rs:1571:23:
start byte index 1 is not a char boundary; it is inside '\u{c0aa7}' (bytes 0..4 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: <regex_lite::string::Match>::as_str
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-lite/src/string.rs:1571:23
   8: repro_regex_lite_src_string_rs_1571_23_f61dec::main
             at ./src/main.rs:7:21
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

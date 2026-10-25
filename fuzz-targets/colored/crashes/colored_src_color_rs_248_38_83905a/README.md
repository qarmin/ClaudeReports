# colored/src/color.rs:248:38

- crate: https://github.com/mackwic/colored @ 5204a26619e10fda0adc77f07588811d6102a40d
- kind: panic
- location: `colored/src/color.rs:248:38`
- last API call: `colored::Color::from_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
end byte index 4 is not a char boundary; it is inside '\u{854ad}' (bytes 2..6 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
colored = { git = "https://github.com/mackwic/colored", rev = "5204a26619e10fda0adc77f07588811d6102a40d" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <colored::Color as ::core::str::FromStr>::from_str("#EB\u{854ad}");
}
```

Run with `cargo run` in this directory (package `repro_colored_src_color_rs_248_38_83905a`).

## Output

```
thread 'main' (521368) panicked at /home/rafal/.cargo/git/checkouts/colored-c4fa3ff01c6a9bba/5204a26/src/color.rs:248:38:
end byte index 4 is not a char boundary; it is inside '\u{854ad}' (bytes 2..6 of string)
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
   7: colored::color::parse_hex
             at /home/rafal/.cargo/git/checkouts/colored-c4fa3ff01c6a9bba/5204a26/src/color.rs:248:38
   8: <colored::color::Color as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/git/checkouts/colored-c4fa3ff01c6a9bba/5204a26/src/color.rs:239:40
   9: repro_colored_src_color_rs_248_38_83905a::main
             at ./src/main.rs:2:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

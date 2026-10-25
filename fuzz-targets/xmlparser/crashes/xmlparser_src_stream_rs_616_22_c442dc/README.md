# xmlparser/src/stream.rs:616:22

- crate: https://github.com/RazrFalcon/xmlparser @ 9c8e34305723118c67d7feba95772705b2247776
- kind: panic
- location: `xmlparser/src/stream.rs:616:22`
- last API call: `xmlparser::Stream::gen_text_pos`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
end byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
xmlparser = { git = "https://github.com/RazrFalcon/xmlparser", rev = "9c8e34305723118c67d7feba95772705b2247776" }
```

`src/main.rs`:

```rust
use xmlparser::Stream;

fn main() {
    let string = "\u{200b}".to_string();
    let stream = Stream::from_substr(&string, 1usize..0usize);
    let _ = stream.gen_text_pos();
}
```

Run with `cargo run` in this directory (package `repro_xmlparser_src_stream_rs_616_22_c442dc`).

## Output

```
thread 'main' (1280268) panicked at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:616:22:
end byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:457:21
   5: <str as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   6: <xmlparser::stream::Stream>::calc_curr_col
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:616:22
   7: <xmlparser::stream::Stream>::gen_text_pos
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:580:19
   8: repro_xmlparser_src_stream_rs_616_22_c442dc::main
             at ./src/main.rs:6:20
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

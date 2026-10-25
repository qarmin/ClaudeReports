# xmlparser/src/stream.rs:292:27

- crate: https://github.com/RazrFalcon/xmlparser @ 9c8e34305723118c67d7feba95772705b2247776
- kind: panic
- location: `xmlparser/src/stream.rs:292:27`
- last API call: `xmlparser::Stream::skip_name`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
start byte index 1 is out of bounds for string of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
xmlparser = { git = "https://github.com/RazrFalcon/xmlparser", rev = "9c8e34305723118c67d7feba95772705b2247776" }
```

`src/main.rs`:

```rust
use xmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 1usize..0usize);
    let mut stream = tokenizer.stream();
    let _ = stream.skip_name();
}
```

Run with `cargo run` in this directory (package `repro_xmlparser_src_stream_rs_292_27_ac2d05`).

## Output

```
thread 'main' (1280324) panicked at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:292:27:
start byte index 1 is out of bounds for string of length 0
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
   7: <xmlparser::stream::Stream>::chars
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:292:27
   8: <xmlparser::stream::Stream>::skip_name
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:447:29
   9: repro_xmlparser_src_stream_rs_292_27_ac2d05::main
             at ./src/main.rs:7:20
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

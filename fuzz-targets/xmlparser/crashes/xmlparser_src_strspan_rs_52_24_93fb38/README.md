# xmlparser/src/strspan.rs:52:24

- crate: https://github.com/RazrFalcon/xmlparser @ 9c8e34305723118c67d7feba95772705b2247776
- kind: panic
- location: `xmlparser/src/strspan.rs:52:24`
- last API call: `xmlparser::Stream::slice_tail`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
end byte index 1 is out of bounds for string of length 0
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
    let string = "".to_string();
    let stream = Stream::from_substr(&string, 0usize..1usize);
    let _ = stream.slice_tail();
}
```

Run with `cargo run` in this directory (package `repro_xmlparser_src_strspan_rs_52_24_93fb38`).

## Output

```
thread 'main' (1280629) panicked at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/strspan.rs:52:24:
end byte index 1 is out of bounds for string of length 0
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
   7: <xmlparser::strspan::StrSpan>::from_substr
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/strspan.rs:52:24
   8: <xmlparser::strspan::StrSpan>::slice_region
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/strspan.rs:89:9
   9: <xmlparser::stream::Stream>::slice_tail
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:304:19
  10: repro_xmlparser_src_strspan_rs_52_24_93fb38::main
             at ./src/main.rs:6:20
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

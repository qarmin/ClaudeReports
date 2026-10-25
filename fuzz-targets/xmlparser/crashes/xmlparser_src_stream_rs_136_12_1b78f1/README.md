# xmlparser/src/stream.rs:136:12

- crate: https://github.com/RazrFalcon/xmlparser @ 9c8e34305723118c67d7feba95772705b2247776
- kind: panic
- location: `xmlparser/src/stream.rs:136:12`
- last API call: `xmlparser::Stream::next_byte`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 1
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
    let stream = Stream::from_substr(&string, 0usize..10000000000000000000usize);
    let _ = stream.next_byte();
}
```

Run with `cargo run` in this directory (package `repro_xmlparser_src_stream_rs_136_12_1b78f1`).

## Output

```
thread 'main' (1280213) panicked at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:136:12:
index out of bounds: the len is 0 but the index is 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <xmlparser::stream::Stream>::next_byte
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:136:12
   4: repro_xmlparser_src_stream_rs_136_12_1b78f1::main
             at ./src/main.rs:6:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

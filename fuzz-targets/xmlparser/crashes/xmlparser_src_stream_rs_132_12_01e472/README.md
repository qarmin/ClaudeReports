# xmlparser/src/stream.rs:132:12

- crate: https://github.com/RazrFalcon/xmlparser @ 9c8e34305723118c67d7feba95772705b2247776
- kind: panic
- location: `xmlparser/src/stream.rs:132:12`
- last API call: `xmlparser::Stream::next_byte`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
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
    let tokenizer = Tokenizer::from_fragment(&string, usize::MAX..0usize);
    let stream = tokenizer.stream();
    let _ = stream.next_byte();
}
```

Run with `cargo run` in this directory (package `repro_xmlparser_src_stream_rs_132_12_01e472`).

## Output

```
thread 'main' (1280519) panicked at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:132:12:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <xmlparser::stream::Stream>::next_byte
             at /home/rafal/.cargo/git/checkouts/xmlparser-00a8851f797f055f/9c8e343/src/stream.rs:132:12
   4: repro_xmlparser_src_stream_rs_132_12_01e472::main
             at ./src/main.rs:7:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

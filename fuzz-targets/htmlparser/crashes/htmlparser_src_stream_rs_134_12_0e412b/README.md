# htmlparser/src/stream.rs:134:12

- crate: https://github.com/jdrouet/htmlparser @ 036a1f71cc277094f34b67d53c353a8634fab91b
- kind: panic
- location: `htmlparser/src/stream.rs:134:12`
- last API call: `htmlparser::Stream::next_byte`
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
htmlparser = { git = "https://github.com/jdrouet/htmlparser", rev = "036a1f71cc277094f34b67d53c353a8634fab91b" }
```

`src/main.rs`:

```rust
use htmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 0usize..1000000000usize);
    let stream = tokenizer.stream();
    let _ = stream.next_byte();
}
```

Run with `cargo run` in this directory (package `repro_htmlparser_src_stream_rs_134_12_0e412b`).

## Output

```
thread 'main' (428644) panicked at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:134:12:
index out of bounds: the len is 0 but the index is 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <htmlparser::stream::Stream>::next_byte
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:134:12
   4: repro_htmlparser_src_stream_rs_134_12_0e412b::main
             at ./src/main.rs:7:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

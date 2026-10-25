# htmlparser/src/strspan.rs:50:9

- crate: https://github.com/jdrouet/htmlparser @ 036a1f71cc277094f34b67d53c353a8634fab91b
- kind: panic
- location: `htmlparser/src/strspan.rs:50:9`
- last API call: `htmlparser::Stream::slice_back`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: start <= end
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
    let tokenizer = Tokenizer::from_fragment(&string, 1usize..0usize);
    let stream = tokenizer.stream();
    let _ = stream.slice_back(1000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_htmlparser_src_strspan_rs_50_9_169e66`).

## Output

```
thread 'main' (428911) panicked at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/strspan.rs:50:9:
assertion failed: start <= end
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <htmlparser::strspan::StrSpan>::from_substr
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/strspan.rs:50:9
   4: <htmlparser::strspan::StrSpan>::slice_region
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/strspan.rs:89:9
   5: <htmlparser::stream::Stream>::slice_back
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:296:19
   6: repro_htmlparser_src_strspan_rs_50_9_169e66::main
             at ./src/main.rs:7:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

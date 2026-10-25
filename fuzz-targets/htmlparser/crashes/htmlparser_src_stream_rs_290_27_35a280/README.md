# htmlparser/src/stream.rs:290:27

- crate: https://github.com/jdrouet/htmlparser @ 036a1f71cc277094f34b67d53c353a8634fab91b
- kind: panic
- location: `htmlparser/src/stream.rs:290:27`
- last API call: `htmlparser::Stream::consume_name`
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
htmlparser = { git = "https://github.com/jdrouet/htmlparser", rev = "036a1f71cc277094f34b67d53c353a8634fab91b" }
```

`src/main.rs`:

```rust
use htmlparser::Tokenizer;

fn main() {
    let string = "".to_string();
    let tokenizer = Tokenizer::from_fragment(&string, 0usize..1usize);
    let mut stream = tokenizer.stream();
    let _ = stream.consume_name();
}
```

Run with `cargo run` in this directory (package `repro_htmlparser_src_stream_rs_290_27_35a280`).

## Output

```
thread 'main' (428851) panicked at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:290:27:
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
   7: <htmlparser::stream::Stream>::chars
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:290:27
   8: <htmlparser::stream::Stream>::skip_name
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:445:29
   9: <htmlparser::stream::Stream>::consume_name
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:427:14
  10: repro_htmlparser_src_stream_rs_290_27_35a280::main
             at ./src/main.rs:7:20
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# htmlparser/src/stream.rs:170:29

- crate: https://github.com/jdrouet/htmlparser @ 036a1f71cc277094f34b67d53c353a8634fab91b
- kind: panic
- location: `htmlparser/src/stream.rs:170:29`
- last API call: `htmlparser::Stream::starts_with`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 1 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
htmlparser = { git = "https://github.com/jdrouet/htmlparser", rev = "036a1f71cc277094f34b67d53c353a8634fab91b" }
```

`src/main.rs`:

```rust
use htmlparser::Stream;

fn main() {
    let string = "".to_string();
    let stream = Stream::from_substr(&string, 0usize..1usize);
    let _ = stream.starts_with(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_htmlparser_src_stream_rs_170_29_6bac37`).

## Output

```
thread 'main' (429155) panicked at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:170:29:
range end index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <[u8] as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <htmlparser::stream::Stream>::starts_with
             at /home/rafal/.cargo/git/checkouts/htmlparser-bd7bc7b721c43315/036a1f7/src/stream.rs:170:29
   6: repro_htmlparser_src_stream_rs_170_29_6bac37::main
             at ./src/main.rs:6:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

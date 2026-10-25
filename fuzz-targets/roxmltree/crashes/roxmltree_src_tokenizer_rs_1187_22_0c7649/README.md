# roxmltree/src/tokenizer.rs:1187:22

- crate: https://github.com/RazrFalcon/roxmltree @ 3f5c78dea57662b43a2f41961397b1f5ffe27166
- kind: panic
- location: `roxmltree/src/tokenizer.rs:1187:22`
- last API call: `roxmltree::Document::text_pos_at`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14387405479267576269

## Message

```
end byte index 1389 is not a char boundary; it is inside '😀' (bytes 1387..1391 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
roxmltree = { git = "https://github.com/RazrFalcon/roxmltree", rev = "3f5c78dea57662b43a2f41961397b1f5ffe27166" }
```

`src/main.rs`:

```rust
use roxmltree::Document;

fn main() {
    let string = include_str!("string_0.txt").to_string();
    let document = Document::parse(&string).unwrap();
    let node = document.root_element();
    let document2 = node.document();
    let _ = document2.text_pos_at(1389usize);
}
```

Run with `cargo run` in this directory (package `repro_roxmltree_src_tokenizer_rs_1187_22_0c7649`).

## Output

```
thread 'main' (317148) panicked at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/tokenizer.rs:1187:22:
end byte index 1389 is not a char boundary; it is inside '😀' (bytes 1387..1391 of string)
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
   6: <roxmltree::tokenizer::Stream>::calc_curr_col
             at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/tokenizer.rs:1187:22
   7: <roxmltree::tokenizer::Stream>::gen_text_pos
             at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/tokenizer.rs:1160:19
   8: <roxmltree::tokenizer::Stream>::gen_text_pos_from
             at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/tokenizer.rs:1171:11
   9: <roxmltree::Document>::text_pos_at
             at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/lib.rs:202:43
  10: repro_roxmltree_src_tokenizer_rs_1187_22_0c7649::main
             at ./src/main.rs:8:23
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# text-size/src/range.rs:431:14

- crate: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814
- kind: panic
- location: `text-size/src/range.rs:431:14`
- source: https://docs.rs/crate/text-size/1.1.1/source/src/range.rs#431
- last API call: `rowan::cursor::SyntaxNode::child_or_token_at_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
TextRange -offset overflowed
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rowan = { git = "https://github.com/rust-analyzer/rowan", rev = "b6f1cb6494a99a2264186d110b1b170989774814" }
```

`src/main.rs`:

```rust
use rowan::GreenNodeBuilder;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let mut green_node_builder = GreenNodeBuilder::new();
    green_node_builder.start_node(syntax_kind);
    green_node_builder.token(syntax_kind, "a");
    green_node_builder.start_node(syntax_kind);
    green_node_builder.finish_node();
    green_node_builder.finish_node();
    let green_node = green_node_builder.finish();
    let syntax_node = SyntaxNode::new_root(green_node.clone());
    let syntax_node2 = syntax_node.first_child().unwrap();
    let syntax_node3 = syntax_node2.parent().unwrap();
    let _ = syntax_node2.child_or_token_at_range(syntax_node3.text_range_without_outer_trivia());
}
```

Run with `cargo run` in this directory (package `repro_text_size_src_range_rs_431_14_1658a4`).

## Output

```
thread 'main' (420558) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/text-size-1.1.1/src/range.rs:431:14:
TextRange -offset overflowed
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<text_size::range::TextRange>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <text_size::range::TextRange as core::ops::arith::Sub<text_size::size::TextSize>>::sub
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/text-size-1.1.1/src/range.rs:431:14
   6: <rowan::cursor::SyntaxNode>::child_or_token_at_range
             at /home/rafal/.cargo/git/checkouts/rowan-6e4cf04043c6ce67/b6f1cb6/src/cursor.rs:653:25
   7: repro_text_size_src_range_rs_431_14_1658a4::main
             at ./src/main.rs:17:26
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
